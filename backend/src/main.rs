use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
    Json,
};
use axum_extra::routing::SpaRouter;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub performance_gain: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Algorithm {
    Nlj,
    Rho,
    // TODO
    Commit(u32), // TODO Uuid
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExperimentType {
    EpcPaging,
    Scalability,
    // TODO
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Dataset {
    CacheFit,
    CacheExceed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Platform {
    Sgx,
    Native,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilingConfiguration {
    algorithm: Algorithm,
    experiment_type: ExperimentType,
    dataset: Dataset,
    platforms: Platform,
    sort_data: bool,
}
#[derive(Debug, Clone)]
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

async fn run_experiment(Json(payload): Json<ProfilingConfiguration>) {
    println!("Received: {:?}", payload);
    // TODO spawn an async task
    let Ok(output) = std::process::Command::new("pwd").output() else {
        println!("Failed to run command!");
        return;
    };
    println!("Output: {:?}", output);
}

#[derive(Debug, Clone)]
struct ServerState {
    commits: Vec<Commit>,
}

impl ServerState {
    fn new() -> Self {
        ServerState {
            commits: vec![],
        }
    }
}

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(ServerState::new()));

    let spa = SpaRouter::new("/assets", "../dist");
    let app = Router::new()
        .merge(spa)
        .route("/api/test", get(|| async {"Test successful!"}))
        .route("/api/commit", post(upload_commit))
        .layer(Extension(state.clone()))
        .route("/api/commit", get(get_commits))
        .layer(Extension(state))
        .route("/api/profiling/", post(run_experiment));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}