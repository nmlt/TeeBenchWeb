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

async fn upload_commit(Extension(state): Extension<Arc<Mutex<ServerState>>>, Json(payload): Json<Commit>) {
    let mut s = state.lock().unwrap();
    s.commits.push(payload);
}

async fn get_commits(Extension(state): Extension<Arc<Mutex<ServerState>>>) -> Json<Value> {
    let s = state.lock().unwrap();
    Json(json!(s.commits))
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
        .layer(Extension(state));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}