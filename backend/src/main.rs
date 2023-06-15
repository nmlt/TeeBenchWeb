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
use tracing::{debug, error, info, instrument, warn};

use backend_lib::{profiling_task, CancelNotifierType, PartialReportType};
use common::commit::{Commit, CommitState};
use common::data_types::{ClientMessage, Job, JobStatus, ServerMessage};

const DEFAULT_TASK_CHANNEL_SIZE: usize = 5;

#[instrument(skip(app_state, payload))]
async fn upload_commit(State(app_state): State<AppState>, Json(payload): Json<Commit>) {
    let mut guard = app_state.commits.lock().unwrap();
    let debug_title = payload.title.clone();
    info!("Received commit: {debug_title}");
    guard.push_commit(payload);
}

#[instrument(skip(app_state))]
async fn get_commits(State(app_state): State<AppState>) -> Json<Value> {
    let guard = app_state.commits.lock().unwrap();
    debug!("Get commits {:#?}", guard);
    Json(json!(*guard.0))
}

#[instrument(skip(app_state, payload))]
#[axum_macros::debug_handler]
async fn run_job(
    State(app_state): State<AppState>,
    Json(payload): Json<Job>,
) -> Result<(), StatusCode> {
    info!("Received: {:?}", payload);
    app_state.worker_task_tx.send(payload).await.unwrap();
    Ok(())
}

#[instrument(skip(app_state))]
async fn get_queue(State(app_state): State<AppState>) -> impl IntoResponse {
    let guard = app_state.queue.lock().unwrap();
    Json(guard.clone())
}

//#[instrument(skip(app_state, ws))]
async fn ws_handler(State(app_state): State<AppState>, ws: WebSocketUpgrade) -> impl IntoResponse {
    info!("ws_handler running.");
    ws.on_upgrade(|socket| {
        handle_socket(
            socket,
            app_state.queue,
            app_state.unqueued_notifier,
            app_state.cancel_notifier,
            app_state.partial_results_receiver,
        )
    })
}

//#[instrument(skip(socket, queue, unqueued_notifier, cancel_notifier))]
async fn handle_socket(
    mut socket: WebSocket,
    queue: Arc<Mutex<VecDeque<Job>>>,
    unqueued_notifier: Arc<tokio::sync::Mutex<mpsc::Receiver<Job>>>,
    cancel_notifier: Arc<tokio::sync::Mutex<mpsc::Sender<CancelNotifierType>>>,
    partial_results_receiver: Arc<tokio::sync::Mutex<mpsc::Receiver<PartialReportType>>>,
) {
    loop {
        let mut unqueued_notifier = unqueued_notifier.lock().await;
        let cancel_notifier = cancel_notifier.lock().await;
        let mut partial_results_receiver = partial_results_receiver.lock().await;
        debug!("Looping back to select socket or queue_state channel receiver");
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
                                    ClientMessage::RemoveAllJobs => {
                                        {
                                            info!("Clearing queue...");
                                            let mut queue = queue.lock().unwrap();
                                            queue.clear();
                                            info!("Queue cleared.");
                                        }
                                        cancel_notifier.send(true).await.unwrap();
                                        info!("Cancelled current job.");
                                    }
                                    ClientMessage::RemoveJob(id) => {
                                        info!("Removing specific job {id:?}...");
                                        let mut done = false;
                                        {
                                            let mut queue = queue.lock().unwrap();
                                            if let Some(found) = queue.iter().position(|j| j.id == id) {
                                                queue.remove(found);
                                                if found != 0 { done = true; }
                                            }
                                        }
                                        if !done {
                                            cancel_notifier.send(true).await.unwrap();
                                        }
                                        info!("Removed job {id:?}.");
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
                            info!("Client disconnected from socket.");
                            return;
                        }
                    }
                } else {
                    error!("Could not receive message on websocket: {msg:?}");
                    return;
                }
            },
            Some(job) = unqueued_notifier.recv() => {
                info!("Queue receiver got a finished job. Notifying client...");
                match job.status {
                    JobStatus::Waiting => {
                        // TODO Remove getting notified of this here?
                    },
                    JobStatus::Done { .. } => {
                        let msg = ServerMessage::RemoveQueueItem(job.clone());
                        let serialized = serde_json::to_vec(&msg).unwrap();
                        if socket.send(Message::Binary(serialized)).await.is_err() {
                            error!("Sending finished queue job to client failed: Client disconnected.");
                            return;
                        }
                    }
                }
            }
            Some((job_id, report)) = partial_results_receiver.recv() => {
                {
                    let mut queue = queue.lock().unwrap();
                    if let Some(j) = queue.iter_mut().find(|j| j.id == job_id) {
                        j.result = Some(common::data_types::JobResult::Exp(Ok(report.clone())));
                    }
                }
                let msg = ServerMessage::PartialReport(job_id, report);
                let serialized = serde_json::to_vec(&msg).unwrap();
                if socket.send(Message::Binary(serialized)).await.is_err() {
                    error!("Sending a partial report to client failed!");
                    return;
                }
            }
        }
    }
}

// TODO Would it be better to just wrap the AppState in an Arc? Still would need the Mutexes on the fields.
#[derive(Debug, Clone, FromRef)]
struct AppState {
    commits: Arc<Mutex<CommitState>>,
    queue: Arc<Mutex<VecDeque<Job>>>,
    unqueued_notifier: Arc<tokio::sync::Mutex<mpsc::Receiver<Job>>>,
    worker_task_tx: Arc<mpsc::Sender<Job>>,
    cancel_notifier: Arc<tokio::sync::Mutex<mpsc::Sender<CancelNotifierType>>>,
    partial_results_receiver: Arc<tokio::sync::Mutex<mpsc::Receiver<PartialReportType>>>,
}

impl AppState {
    fn new(
        commits: Arc<Mutex<CommitState>>,
        queue: Arc<Mutex<VecDeque<Job>>>,
        unqueued_notifier: Arc<tokio::sync::Mutex<mpsc::Receiver<Job>>>,
        worker_task_tx: Arc<mpsc::Sender<Job>>,
        cancel_notifier: Arc<tokio::sync::Mutex<mpsc::Sender<CancelNotifierType>>>,
        partial_results_receiver: Arc<tokio::sync::Mutex<mpsc::Receiver<PartialReportType>>>,
    ) -> Self {
        AppState {
            commits,
            queue,
            unqueued_notifier,
            worker_task_tx,
            cancel_notifier,
            partial_results_receiver,
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let commits: Arc<Mutex<CommitState>>;
    let queue: Arc<Mutex<VecDeque<Job>>>;
    if cfg!(feature = "static") {
        commits = Arc::new(Mutex::new(CommitState::new(vec![])));
        queue = Arc::new(Mutex::new(VecDeque::new()));
    } else {
        let mut hardcoded_commits = vec![common::hardcoded::predefined_commit()];
        let (mut append_commits, append_jobs) = common::hardcoded::hardcoded_commits();
        hardcoded_commits.append(&mut append_commits);
        commits = Arc::new(Mutex::new(CommitState::new(hardcoded_commits)));
        queue = Arc::new(Mutex::new(VecDeque::from(append_jobs)));
    }
    let (queue_tx, queue_rx) = mpsc::channel(DEFAULT_TASK_CHANNEL_SIZE);
    let (profiling_tx, profiling_rx) = mpsc::channel(DEFAULT_TASK_CHANNEL_SIZE);
    let (cancel_tx, cancel_rx) = mpsc::channel(DEFAULT_TASK_CHANNEL_SIZE);
    let (partial_results_sender, partial_results_receiver) =
        mpsc::channel(DEFAULT_TASK_CHANNEL_SIZE);

    tokio::spawn(profiling_task(
        Arc::clone(&commits),
        Arc::clone(&queue),
        queue_tx,
        profiling_rx,
        cancel_rx,
        Arc::new(tokio::sync::Mutex::new(partial_results_sender)),
    ));

    let app_state = AppState::new(
        commits,
        queue,
        Arc::new(tokio::sync::Mutex::new(queue_rx)),
        Arc::new(profiling_tx),
        Arc::new(tokio::sync::Mutex::new(cancel_tx)),
        Arc::new(tokio::sync::Mutex::new(partial_results_receiver)),
    );

    let spa = SpaRouter::new("/assets", "../dist"); // TODO Remove and use the tower middleware instead.
    let app = Router::new()
        .merge(spa)
        .route("/api/test", get(|| async { "Test successful!" }))
        .route("/api/commit", post(upload_commit))
        .with_state(app_state.clone())
        .route("/api/commit", get(get_commits))
        .with_state(app_state.clone())
        .route("/api/job", post(run_job))
        .with_state(app_state.clone())
        .route("/api/ws", get(ws_handler))
        .with_state(app_state.clone())
        .route("/api/queue", get(get_queue))
        .with_state(app_state);

    info!("Listening on 0.0.0.0:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
