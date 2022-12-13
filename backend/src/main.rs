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
use tokio::{
    sync::{mpsc, oneshot},
    time::{sleep, Duration},
};
use tracing::{info, instrument, warn};

use common::data_types::{Commit, Job, ProfilingConfiguration, QueueMessage};

const DEFAULT_TASK_CHANNEL_SIZE: usize = 5;

async fn upload_commit(State(state): State<Arc<Mutex<ServerState>>>, Json(payload): Json<Commit>) {
    let mut s = state.lock().unwrap();
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

#[instrument(skip(rx))]
async fn profiling_task(
    rx: mpsc::Receiver<ProfilingConfiguration>,
    queue: Arc<Mutex<VecDeque<ProfilingConfiguration>>>,
    queue_tx: mpsc::Sender<Job>,
) {
    // Using a tokio Mutex here to make it Send. Which is required...
    let rx = Arc::new(tokio::sync::Mutex::new(rx));
    #[instrument(skip(queue, oneshot_tx))]
    async fn work_on_queue(
        queue: Arc<Mutex<VecDeque<ProfilingConfiguration>>>,
        oneshot_tx: oneshot::Sender<()>,
        queue_tx: mpsc::Sender<Job>,
    ) {
        fn pop_queue(
            queue: Arc<Mutex<VecDeque<ProfilingConfiguration>>>,
        ) -> Option<ProfilingConfiguration> {
            let mut guard = queue.lock().unwrap();
            guard.pop_front()
        }
        // TODO Do not clone the queue. It should be always the same arc, but how to make it work with the borrow checker?
        // Probably by making the above function a closure, which would also be more idiomatic.
        while let Some(current_conf) = pop_queue(queue.clone()) {
            info!("Working on {current_conf:#?}...");
            let out = tokio::process::Command::new("sleep")
                .arg("5")
                .output()
                .await;
            info!("Process completed with {out:?}.");
            let finished_job = Job::Finished {
                config: current_conf,
                result: None,
            };
            queue_tx.send(finished_job).await.unwrap();
        }
        oneshot_tx.send(()).unwrap();
    }
    async fn receive_confs(
        queue: Arc<Mutex<VecDeque<ProfilingConfiguration>>>,
        rx: Arc<tokio::sync::Mutex<mpsc::Receiver<ProfilingConfiguration>>>,
        queue_tx: mpsc::Sender<Job>,
    ) {
        let mut rx_guard = rx.lock().await;
        match rx_guard.recv().await {
            Some(conf) => {
                info!("New task came in!");
                let running_job = Job::Running(conf.clone());
                queue_tx.send(running_job).await.unwrap();
                let mut guard = queue.lock().unwrap();
                guard.push_back(conf);
            }
            None => warn!("Received None!"), // TODO Why would this happen?
        }
    }
    loop {
        receive_confs(queue.clone(), rx.clone(), queue_tx.clone()).await;
        let (work_finished_tx, mut work_finished_rx) = oneshot::channel();
        tokio::spawn(work_on_queue(
            queue.clone(),
            work_finished_tx,
            queue_tx.clone(),
        ));
        loop {
            // Following the advice in the tokio::oneshot documentation to make the rx &mut.
            tokio::select! {
                _ = receive_confs(queue.clone(), rx.clone(), queue_tx.clone()) => {},
                _ = &mut work_finished_rx => { break; },
            }
        }
    }
}

#[instrument]
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
                                        sleep(Duration::from_millis(100)).await;
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
                    Job::Finished { config, result } => {
                        if socket.send(
                            Message::Binary(
                                serde_json::to_vec(
                                    &QueueMessage::RemoveQueueItem(Job::Finished {
                                        config, 
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
