use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Extension,
    },
    http::StatusCode,
    routing::{get, post},
    Json, Router,
    response::IntoResponse,
};
use axum_extra::routing::SpaRouter;
use tracing::{info, instrument, warn};

use tokio::sync::{mpsc, oneshot, broadcast};

use serde_json::{json, Value};
use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;

use common::data_types::{Commit, ProfilingConfiguration, Report, Job};

const DEFAULT_TASK_CHANNEL_SIZE: usize = 5;

async fn upload_commit(
    Extension(state): Extension<Arc<Mutex<ServerState>>>,
    Json(payload): Json<Commit>,
) {
    let mut s = state.lock().unwrap();
    s.commits.push(payload);
}

async fn get_commits(Extension(state): Extension<Arc<Mutex<ServerState>>>) -> Json<Value> {
    let s = state.lock().unwrap();
    Json(json!(s.commits))
}

#[instrument(skip(profiling_state, payload))]
#[axum_macros::debug_handler]
async fn run_experiment(
    Extension(profiling_state): Extension<Arc<ProfilingState>>,
    Json(payload): Json<ProfilingConfiguration>,
) -> Result<(), StatusCode> {
    info!("Received: {:?}", payload);
    profiling_state.channel_tx.send(payload).await.unwrap();
    Ok(())
}

#[instrument(skip(rx))]
async fn profiling_task(rx: mpsc::Receiver<ProfilingConfiguration>, queue: Arc<RwLock<VecDeque<ProfilingConfiguration>>>, queue_tx: broadcast::Sender<bool>) {
    // Using a tokio Mutex here to make it Send. Which is required...
    let rx = Arc::new(tokio::sync::Mutex::new(rx));
    #[instrument(skip(queue, oneshot_tx))]
    async fn work_on_queue(
        queue: Arc<RwLock<VecDeque<ProfilingConfiguration>>>,
        oneshot_tx: oneshot::Sender<()>,
    ) {
        fn pop_queue(
            queue: Arc<RwLock<VecDeque<ProfilingConfiguration>>>,
        ) -> Option<ProfilingConfiguration> {
            let mut guard = queue.write().unwrap();
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
        }
        oneshot_tx.send(()).unwrap();
    }
    async fn receive_confs(
        queue: Arc<RwLock<VecDeque<ProfilingConfiguration>>>,
        rx: Arc<tokio::sync::Mutex<mpsc::Receiver<ProfilingConfiguration>>>,
    ) {
        let mut rx_guard = rx.lock().await;
        match rx_guard.recv().await {
            Some(conf) => {
                info!("New task came in!");
                let mut guard = queue.write().unwrap();
                guard.push_back(conf);
            }
            None => warn!("Received None!"), // TODO Why would this happen?
        }
    }
    loop {
        receive_confs(queue.clone(), rx.clone()).await;
        let (work_finished_tx, mut work_finished_rx) = oneshot::channel();
        tokio::spawn(work_on_queue(queue.clone(), work_finished_tx));
        loop {
            // Following the advice in the tokio::oneshot documentation to make the rx &mut.
            tokio::select! {
                _ = receive_confs(queue.clone(), rx.clone()) => {},
                _ = &mut work_finished_rx => { break; },
            }
        }
    }
}

#[instrument]
async fn ws_handler(Extension(queue_state): Extension<QueueState>, ws: WebSocketUpgrade) -> impl IntoResponse {
    info!("ws_handler running.");
    ws.on_upgrade(handle_socket)
}

#[instrument]
async fn handle_socket(mut socket: WebSocket) {
    loop {
        if let Some(msg) = socket.recv().await {
            if let Ok(msg) = msg {
                match msg {
                    Message::Text(t) => {
                        println!("client sent str: {:?}", t);
                    }
                    Message::Binary(_) => {
                        println!("client sent binary data");
                    }
                    Message::Ping(_) => {
                        println!("socket ping");
                    }
                    Message::Pong(_) => {
                        println!("socket pong");
                    }
                    Message::Close(_) => {
                        println!("client disconnected");
                        return;
                    }
                }
            } else {
                println!("client disconnected");
                return;
            }
        }
        if socket
            .send(Message::Text(String::from("Hi!")))
            .await
            .is_err()
        {
            println!("client disconnected");
            return;
        }
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
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

#[derive(Debug, Clone)]
struct QueueState {
    queue: Arc<RwLock<VecDeque<ProfilingConfiguration>>>,
    /// Channel receiver to get notified if new items were queued or unqueued.
    queue_rx: Arc<broadcast::Receiver<bool>>,
    // TODO Some kind of handle or channel that receives a handle to cancel the current queue item.
}

#[tokio::main]
async fn main() {
    // If I use Level::DEBUG, I get lots of log messages from hyper/mio/etc.
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let (profiling_tx, profiling_rx) = mpsc::channel(DEFAULT_TASK_CHANNEL_SIZE);
    let queue = Arc::new(RwLock::new(VecDeque::new()));
    let (queue_tx, queue_rx) = broadcast::channel(DEFAULT_TASK_CHANNEL_SIZE);
    tokio::spawn(profiling_task(profiling_rx, queue.clone(), queue_tx));
    let state = Arc::new(Mutex::new(ServerState::default()));
    let profiling_state = Arc::new(ProfilingState {
        channel_tx: profiling_tx.clone(),
    });
    let queue_state = QueueState {
        queue,
        queue_rx: Arc::new(queue_rx),
    };

    let spa = SpaRouter::new("/assets", "../dist");
    let app = Router::new()
        .merge(spa)
        .route("/api/test", get(|| async { "Test successful!" }))
        .route("/api/commit", post(upload_commit))
        .layer(Extension(state.clone()))
        .route("/api/commit", get(get_commits))
        .layer(Extension(state.clone()))
        .route("/api/profiling/", post(run_experiment))
        .layer(Extension(profiling_state))
        .route("/api/queue", get(ws_handler))
        .layer(Extension(queue_state));

    info!("Listening on 0.0.0.0:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
