use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
    Json,
    http::StatusCode
};
use axum_extra::routing::SpaRouter;
use tracing::{info, warn, instrument};

use tokio::sync::mpsc;

use chrono::{DateTime, offset::Utc};
use serde::{Serialize, Deserialize};
use serde_json::{Value, json};
use std::sync::Arc;
use std::sync::Mutex;

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
    Nlj,
    Rho,
    // TODO
    Commit(u32), // TODO Uuid
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum ExperimentType {
    #[default]
    EpcPaging,
    Scalability,
    // TODO
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
    platforms: Platform,
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

async fn upload_commit(Extension(state): Extension<Arc<Mutex<ServerState>>>, Json(payload): Json<Commit>) {
    let mut s = state.lock().unwrap();
    s.commits.push(payload);
}

async fn get_commits(Extension(state): Extension<Arc<Mutex<ServerState>>>) -> Json<Value> {
    let s = state.lock().unwrap();
    Json(json!(s.commits))
}

#[instrument]
#[axum_macros::debug_handler]
async fn run_experiment(Extension(state): Extension<Arc<ProfilingState>>, Json(payload): Json<ProfilingConfiguration>) -> Result<(), StatusCode> {
    info!("Received: {:?}", payload);
    state.channel_tx.send(payload).await.unwrap();
    Ok(())
}

#[instrument]
async fn profiling_task(mut rx: mpsc::Receiver<ProfilingConfiguration>) {
    while let Some(conf) = rx.recv().await {
        info!("Starting work on {:?}...", conf);
        let Ok(output) = std::process::Command::new("pwd").output() else {
            warn!("Failed to run command!");
            return;
        };
        info!("Output: {:?}", output);
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

    let (profiling_tx, profiling_rx) = mpsc::channel(5);
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