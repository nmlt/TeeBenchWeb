use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
    Json,
    http::StatusCode
};
use axum_extra::routing::SpaRouter;
use tracing::{info, warn, instrument};

use tokio::sync::{oneshot, mpsc};

use chrono::{DateTime, offset::Utc};
use serde::{Serialize, Deserialize};
use serde_json::{Value, json};
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    pub title: String,
    pub datetime: DateTime<Utc>,
    pub code: String,
    pub report: Option<Report>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Report {
    pub performance_gain: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum Algorithm {
    #[default]
    Rho,
    Cht,
    Commit(u32), // TODO Uuid
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum ExperimentType {
    #[default]
    EpcPaging,
    Throughput,
    CpuCyclesTuple,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum Dataset {
    #[default]
    CacheFit,
    CacheExceed,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum Platform {
    #[default]
    Sgx,
    Native,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProfilingConfiguration {
    algorithm: Algorithm,
    experiment_type: ExperimentType,
    dataset: Dataset,
    platform: Platform,
    sort_data: bool,
}
#[derive(Debug, Clone, Default)]
pub struct Job {
    config: ProfilingConfiguration,
    result: Option<Report>,
}

impl Job {
    fn with_config(config: ProfilingConfiguration) -> Self {
        Self {
            config,
            result: None,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

const PROFILING_TASK_CHANNEL_SIZE: usize = 5;

async fn upload_commit(Extension(state): Extension<Arc<Mutex<ServerState>>>, Json(payload): Json<Commit>) {
    let mut s = state.lock().unwrap();
    s.commits.push(payload);
}

async fn get_commits(Extension(state): Extension<Arc<Mutex<ServerState>>>) -> Json<Value> {
    let s = state.lock().unwrap();
    Json(json!(s.commits))
}

#[instrument(skip(profiling_state))]
#[axum_macros::debug_handler]
async fn run_experiment(Extension(profiling_state): Extension<Arc<ProfilingState>>, Json(payload): Json<ProfilingConfiguration>) -> Result<(), StatusCode> {
    info!("Received: {:?}", payload);
    profiling_state.channel_tx.send(payload).await.unwrap();
    Ok(())
}

#[instrument(skip(rx))]
async fn profiling_task(rx: mpsc::Receiver<ProfilingConfiguration>) {
    let queue = Arc::new(Mutex::new(VecDeque::new()));
    // Using a tokio's Mutex here to make it Send. Which is required...
    let rx = Arc::new(tokio::sync::Mutex::new(rx));
    #[instrument(skip(queue, oneshot_tx))]
    async fn work_on_queue(queue: Arc<Mutex<VecDeque<ProfilingConfiguration>>>, oneshot_tx: oneshot::Sender<()>) {
        fn pop_queue(queue: Arc<Mutex<VecDeque<ProfilingConfiguration>>>) -> Option<ProfilingConfiguration> {
            let mut guard = queue.lock().unwrap();
            guard.pop_front()
        }
        // TODO Do not clone the queue. It should be always the same arc, but how to make it work with the borrow checker?
        // Probably by making the above function a closure, which would also be more idiomatic.
        while let Some(current_conf) = pop_queue(queue.clone()) {
            info!("Working on {current_conf:#?}...");
            let out = tokio::process::Command::new("sleep").arg("5").output().await;
            info!("Process completed with {out:?}.");
        }
        oneshot_tx.send(()).unwrap();
    }
    async fn receive_confs(queue: Arc<Mutex<VecDeque<ProfilingConfiguration>>>, rx: Arc<tokio::sync::Mutex<mpsc::Receiver<ProfilingConfiguration>>>) {
        let mut rx_guard = rx.lock().await;
        match rx_guard.recv().await {
            Some(conf) => {
                info!("New task came in!");
                let mut guard = queue.lock().unwrap();
                guard.push_back(conf);
            },
            None => warn!("Received None!"), // TODO Why would this happen?
        }
    }
    loop {
        receive_confs(queue.clone(), rx.clone()).await;
        let (work_finished_tx, mut work_finished_rx) = oneshot::channel();
        tokio::spawn(work_on_queue(queue.clone(), work_finished_tx));
        loop {
            // Following the advice in the tokio::oneshot documentation to make the rx &mut.
            tokio::select!{
                _ = receive_confs(queue.clone(), rx.clone()) => {},
                _ = &mut work_finished_rx => { break; },
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
    channel_tx: mpsc::Sender<ProfilingConfiguration>,
}

#[tokio::main]
async fn main() {
    // If I use Level::DEBUG, I get lots of log messages from hyper/mio/etc.
    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();

    let (profiling_tx, profiling_rx) = mpsc::channel(PROFILING_TASK_CHANNEL_SIZE);
    tokio::spawn(profiling_task(profiling_rx));
    let state = Arc::new(Mutex::new(ServerState::default()));
    let profiling_state = Arc::new(ProfilingState { channel_tx: profiling_tx.clone() });

    let spa = SpaRouter::new("/assets", "../dist");
    let app = Router::new()
        .merge(spa)
        .route("/api/test", get(|| async {"Test successful!"}))
        .route("/api/commit", post(upload_commit))
        .layer(Extension(state.clone()))
        .route("/api/commit", get(get_commits))
        .layer(Extension(state.clone()))
        .route("/api/profiling/", post(run_experiment))
        .layer(Extension(profiling_state));

    info!("Listening on 0.0.0.0:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}