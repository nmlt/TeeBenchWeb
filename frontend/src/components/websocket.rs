use std::rc::Rc;
use yew::platform::pinned::mpsc::{unbounded, UnboundedSender};
use yew::prelude::*;
use yewdux::prelude::*;

use common::commit::CompilationStatus;
use common::data_types::{
    ClientMessage, Job, JobConfig, JobResult, JobStatus, PerfReportConfig, ServerMessage,
};
use futures::{SinkExt, StreamExt};
use gloo_console::log;
use gloo_net::websocket::{futures::WebSocket, Message};
use wasm_bindgen_futures::spawn_local;

use crate::job_results_view::FinishedJobState;
use crate::queue::QueueState;
use common::commit::{CommitState, PerfReportStatus};

// Idea: Use a struct component to tap into the component lifecycle: create to establish the websocket connection, and update to send and receive (eg. async clock example in yew sends itself a message if something arrives on a channel, so that should also be possible for a websocket). Now the question is, how do I get the other components (Commits, PerfReport, Profiling) to communicate with this struct component?
// Easiest would be if they could send messages to the component. Maybe by passing a callback around? Or just use a channel. I could store the transmitter part in a hook and pass the receiver part as props to the struct component.

#[derive(Debug, Clone, Default, Store)]
pub struct WebsocketState {
    pub transmitter: Option<Rc<UnboundedSender<ClientMessage>>>,
}

impl PartialEq for WebsocketState {
    fn eq(&self, other: &Self) -> bool {
        self.transmitter.is_some() && other.transmitter.is_some()
    }
}

impl WebsocketState {
    fn new(tx: UnboundedSender<ClientMessage>) -> Self {
        Self {
            transmitter: Some(Rc::new(tx)),
        }
    }
    pub fn send(&self, msg: ClientMessage) {
        if let Some(tx) = &self.transmitter {
            tx.send_now(msg).unwrap();
        } else {
            panic!();
        }
    }
}

// Idea: create channel in this component, put transmitter into hook (maybe use_state instead of use_store if that helps with PartialEq requirement?). receiver is moved into the use_effect_with_deps (no deps for only on first render) hook. in there I spawn_local, once for the receiver: whenever it receives, i write to the websocket. The other spawn_local receives from the websocket and writes the received data into the appropriate hooks.
#[function_component]
pub fn Websocket() -> Html {
    log!("Establishing Websocket...");
    let (tx, mut rx) = unbounded::<ClientMessage>();
    let dispatch = Dispatch::<WebsocketState>::new();
    let commit_dispatch = Dispatch::<CommitState>::new();
    dispatch.set(WebsocketState::new(tx));
    use_effect_with_deps(
        move |_| {
            use common::commit::Commit;
            use gloo_net::http::{Method, Request};
            let commit_dispatch = commit_dispatch.clone();
            spawn_local(async move {
                let commit_dispatch = commit_dispatch.clone();
                let resp: Result<Vec<Commit>, _> = Request::get("/api/commit")
                    .method(Method::GET)
                    .send()
                    .await
                    .expect("Server didn't respond. Is it running?")
                    .json()
                    .await;
                //log!(format!("GET /api/commits: Response: {:?}", resp));
                match resp {
                    Ok(json) => {
                        //log!(format!("got commits: {json:?}"));
                        let commit_store = CommitState::new(json);
                        commit_dispatch.set(commit_store);
                    }
                    Err(e) => log!("Error getting commit json: ", e.to_string()),
                }
                let queue_dispatch = Dispatch::<QueueState>::new();
                let resp: Result<Vec<Job>, _> = Request::get("/api/queue")
                    .method(Method::GET)
                    .send()
                    .await
                    .expect("Server didn't respond. Is it running?")
                    .json()
                    .await;
                match resp {
                    Ok(json) => {
                        log!(format!("Got queue: {json:?}"));
                        let queue_state = QueueState::new(json);
                        queue_dispatch.set(queue_state);
                    }
                    Err(e) => log!("Error getting queue json: ", e.to_string()),
                }
            });
            // TODO Find a way for the frontend to get the websites actual address (not localhost)
            let ws = match WebSocket::open("ws://localhost:3000/api/ws") {
                // `ws://` is required, otherwise there's an error.
                Ok(ws) => ws,
                Err(e) => {
                    log!(format!("Error opening websocket: {:?}", e));
                    panic!();
                }
            };
            let (mut write, mut read) = ws.split();

            spawn_local(async move {
                while let Some(msg) = rx.next().await {
                    write
                        .send(Message::Bytes(serde_json::to_vec(&msg).unwrap()))
                        .await
                        .unwrap();
                }
            });
            spawn_local(async move {
                let queue_state_dispatch = Dispatch::<QueueState>::new();
                let finished_job_dispatch = Dispatch::<FinishedJobState>::new();
                let commit_dispatch = Dispatch::<CommitState>::new();
                while let Some(Ok(Message::Bytes(msg))) = read.next().await {
                    let msg = serde_json::from_slice(&msg).unwrap();
                    log!(format!("Got msg {msg:#?}"));
                    match msg {
                        ServerMessage::RemoveQueueItem(finished_job) => match finished_job.config {
                            JobConfig::Profiling(_) => {
                                finished_job_dispatch.reduce_mut(|finished_job_state| {
                                    finished_job_state.jobs.push(finished_job.clone());
                                });
                                queue_state_dispatch.reduce_mut(|queue_state| {
                                    if queue_state.queue.pop_front().is_none() {
                                        log!("Error: Queue out of sync! Reload the page?");
                                    }
                                });
                            }
                            JobConfig::PerfReport(pr_conf) => {
                                commit_dispatch.reduce_mut(|commit_store| {
                                    let commit = commit_store.get_by_id_mut(&pr_conf.id);
                                    if let Some(mut commit) = commit {
                                        if let JobStatus::Done { .. } = finished_job.status {
                                            commit.perf_report_running =
                                                if finished_job.result.is_some() {
                                                    PerfReportStatus::Successful
                                                } else {
                                                    PerfReportStatus::Failed
                                                };
                                            commit.report = finished_job.result;
                                        } else {
                                            log!("Error: Got an unfinished job in the websocket.");
                                        }
                                    }
                                });
                            }
                            JobConfig::Compile(ref id) => {
                                commit_dispatch.reduce_mut(|commit_store| {
                                    let commit = commit_store.get_by_id_mut(id);
                                    if let Some(mut commit) = commit {
                                        if let JobStatus::Done { .. } = finished_job.status {
                                            if let Some(JobResult::Compile(r)) = finished_job.result {
                                                match r {
                                                    Ok(msg) => commit.compilation = CompilationStatus::Successful(msg),
                                                    Err(e) => commit.compilation = CompilationStatus::Failed(e),
                                                }
                                            } else {
                                                log!("Error: Got a job result for something else than compiling when expecting Compile.")
                                            }
                                        } else {
                                            log!("Error: Got an unfinished job in the websocket.");
                                        }
                                    }
                                });
                            }
                        },
                        ServerMessage::PartialReport(job_id, report) => {
                            queue_state_dispatch.reduce_mut(|queue_state| {
                                if let Some(mut job) = queue_state.queue.iter_mut().find(|j| j.id == job_id) {
                                    let res = Some(JobResult::Exp(Ok(report)));
                                    job.result = res.clone();
                                    if let JobConfig::PerfReport(PerfReportConfig { id, ..}) =  job.config {
                                        commit_dispatch.reduce_mut(|commit_state| {
                                            if let Some(commit) = commit_state.get_by_id_mut(&id) {
                                                commit.report = res;
                                            } else {
                                                log!("Error: Commits out of sync with jobs!");
                                            }
                                        });
                                    }
                                } else {
                                    log!("Error: Queue out of sync! Partial job result's id not in queue. Reload the page?");
                                }
                            });
                        }
                    }
                }
                //log!("Done!");
            });
        },
        (),
    );
    html! {
        <span class="position-absolute top-0 start-100 translate-middle badge rounded-pill bg-info">
            {" "}
            <span class="visually-hidden">{"websocket status"}</span>
        </span>
    }
}
