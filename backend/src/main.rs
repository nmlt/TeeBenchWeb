use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        FromRef, State,
    },
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use axum_extra::routing::SpaRouter;
use serde_json::{json, Value};
use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::mpsc;
use tracing::{info, instrument, warn};

use common::data_types::{
    Commit, Job, ProfilingConfiguration, QueueMessage,
};
use backend_lib::profiling_task;

const DEFAULT_TASK_CHANNEL_SIZE: usize = 5;

#[instrument(skip(state, payload))]
async fn upload_commit(State(state): State<Arc<Mutex<ServerState>>>, Json(payload): Json<Commit>) {
    let mut s = state.lock().unwrap();
    let debug_title = payload.title.clone();
    info!("Received commit: {debug_title}");
    s.commits.push(payload);
}

async fn get_commits(State(state): State<Arc<Mutex<ServerState>>>) -> Json<Value> {
    let s = state.lock().unwrap();
    Json(json!(s.commits))
}

#[instrument(skip(profiling_state, payload))]
#[axum_macros::debug_handler]
async fn run_experiment(
    State(profiling_state): State<Arc<ProfilingState>>,
    Json(payload): Json<ProfilingConfiguration>,
) -> Result<(), StatusCode> {
    info!("Received: {:?}", payload);
    profiling_state.channel_tx.send(payload).await.unwrap();
    Ok(())
}

#[instrument(skip(queue_state, ws))]
async fn ws_handler(
    State(queue_state): State<Arc<QueueState>>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    info!("ws_handler running.");
    ws.on_upgrade(|socket| handle_socket(socket, queue_state))
}

#[instrument(skip(socket, queue_state))]
async fn handle_socket(mut socket: WebSocket, queue_state: Arc<QueueState>) {
    loop {
        let mut guard = queue_state.queue_rx.lock().await;
        // TODO Check if data loss could happen due to cancelation.
        tokio::select! {
            Some(msg) = socket.recv() => {
                info!("Socket received.");
                if let Ok(msg) = msg {
                    match msg {
                        Message::Text(t) => {
                            warn!("Client sent str: {:?}.", t);
                        }
                        Message::Binary(b) => {
                            info!("Client sent binary data.");
                            if let Ok(request) = serde_json::from_slice(&b) {
                                match request {
                                    QueueMessage::RequestQueue => {
                                        let queue = {
                                            // TODO This is probably not required to immediately drop the read_guard.
                                            info!("Locking queue_state...");
                                            queue_state.queue.lock().unwrap().clone()
                                        };
                                        for item in queue {
                                            if socket
                                                .send(Message::Binary(
                                                    serde_json::to_vec(&QueueMessage::AddQueueItem(
                                                        item,
                                                    ))
                                                    .unwrap(),
                                                ))
                                                .await
                                                .is_err()
                                            {
                                                info!("Client disconnected while sending queue items.");
                                                return;
                                            }
                                        }
                                        info!("Sent queue to frontend!");
                                    }
                                    QueueMessage::RequestClear => {
                                        // Cancel current job and clear queue
                                        warn!("Unimplemented clear queue request received.");
                                    }
                                    QueueMessage::Acknowledge => {
                                        // TODO I don't think I need this.
                                        warn!("Acknowledge received.");
                                    }
                                    QueueMessage::AddQueueItem(_)
                                    | QueueMessage::RemoveQueueItem(_) => {
                                        // TODO The frontend shouldn't sent those messages!
                                        warn!("Currently not supported Message received.");
                                    }
                                }
                            }
                        }
                        Message::Ping(_) => {
                            info!("Socket ping.");
                        }
                        Message::Pong(_) => {
                            info!("Socket pong.");
                        }
                        Message::Close(_) => {
                            info!("Client disconnected.");
                            return;
                        }
                    }
                } else {
                    info!("Client disconnected.");
                    return;
                }
            },
            Some(job) = guard.recv() => {
                info!("Queue receiver got a new running or finished job.");
                match job {
                    Job::Running(c) => {
                        if socket.send(Message::Binary(serde_json::to_vec(&QueueMessage::AddQueueItem(c)).unwrap())).await.is_err() {
                            info!("Client disconnected.");
                            return;
                        }
                    },
                    Job::Finished { config, submitted, runtime, result } => {
                        if socket.send(
                            Message::Binary(
                                serde_json::to_vec(
                                    &QueueMessage::RemoveQueueItem(Job::Finished {
                                        config,
                                        submitted,
                                        runtime,
                                        result
                                    })
                                )
                                .unwrap()
                            )).await
                            .is_err() {
                            info!("Client disconnected.");
                            return;
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
struct ServerState {
    commits: Vec<Commit>,
}

#[derive(Debug, Clone)]
struct ProfilingState {
    /// Channel transmitter to send newly arrived Jobs to the async worker task (which has the corresponding receiver).
    channel_tx: mpsc::Sender<ProfilingConfiguration>,
}

#[derive(Debug)]
struct QueueState {
    queue: Arc<Mutex<VecDeque<ProfilingConfiguration>>>,
    /// Channel receiver to get notified if new items were queued or unqueued.
    // TODO There's only one receiver, right?
    queue_rx: Arc<tokio::sync::Mutex<mpsc::Receiver<Job>>>,
    // TODO Some kind of handle or channel that receives a handle to cancel the current queue item.
}

#[derive(Debug, Clone, FromRef)]
struct AppState {
    state: Arc<Mutex<ServerState>>,
    profiling_state: Arc<ProfilingState>,
    queue_state: Arc<QueueState>,
}

#[tokio::main]
async fn main() {
    // If I use Level::DEBUG, I get lots of log messages from hyper/mio/etc.
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let (profiling_tx, profiling_rx) = mpsc::channel(DEFAULT_TASK_CHANNEL_SIZE);
    let queue = Arc::new(Mutex::new(VecDeque::new()));
    let (queue_tx, queue_rx) = mpsc::channel(DEFAULT_TASK_CHANNEL_SIZE);
    tokio::spawn(profiling_task(profiling_rx, queue.clone(), queue_tx));
    let state = Arc::new(Mutex::new(ServerState::default()));
    let profiling_state = Arc::new(ProfilingState {
        channel_tx: profiling_tx.clone(),
    });
    let queue_state = QueueState {
        queue,
        queue_rx: Arc::new(tokio::sync::Mutex::new(queue_rx)),
    };
    let app_state = AppState {
        state,
        profiling_state,
        queue_state: Arc::new(queue_state),
    };

    let spa = SpaRouter::new("/assets", "../dist");
    let app = Router::new()
        .merge(spa)
        .route("/api/test", get(|| async { "Test successful!" }))
        .route("/api/commit", post(upload_commit))
        .with_state(app_state.clone())
        .route("/api/commit", get(get_commits))
        .with_state(app_state.clone())
        .route("/api/profiling", post(run_experiment))
        .with_state(app_state.clone())
        .route("/api/queue", get(ws_handler))
        .with_state(app_state);

    info!("Listening on 0.0.0.0:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
