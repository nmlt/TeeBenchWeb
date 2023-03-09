use std::collections::VecDeque;
use std::rc::Rc;
use yew::platform::pinned::mpsc::{unbounded, UnboundedReceiver, UnboundedSender};
use yew::prelude::*;
use yewdux::prelude::*;

use common::data_types::{ClientMessage, ServerMessage};
use futures::{SinkExt, StreamExt};
use gloo_console::log;
use gloo_net::websocket::{futures::WebSocket, Message};
use wasm_bindgen_futures::spawn_local;

use crate::job_results_view::FinishedJobState;
use crate::queue::QueueState;

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
    dispatch.set(WebsocketState::new(tx));
    use_effect_with_deps(
        move |_| {
            // TODO Change address to '/api/ws' because its not just the queue being transmitted now.
            // TODO Find a way for the frontend to get the websites actual address (not localhost)
            let ws = match WebSocket::open("ws://localhost:3000/api/queue") {
                // `ws://` is required, otherwise there's an error.
                Ok(ws) => ws,
                Err(e) => {
                    log!(format!("Error opening websocket: {:?}", e));
                    panic!();
                }
            };
            let (mut write, mut read) = ws.split();

            spawn_local(async move {
                //log!("sending first...");
                write
                    .send(Message::Bytes(
                        serde_json::to_vec(&ClientMessage::RequestQueue).unwrap(),
                    ))
                    .await
                    .unwrap();
                //log!("Done!...");
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
                while let Some(Ok(Message::Bytes(msg))) = read.next().await {
                    let msg = serde_json::from_slice(&msg).unwrap();
                    log!(format!("Got msg {msg:?}"));
                    match msg {
                        ServerMessage::AddQueueItem(conf) => {
                            queue_state_dispatch.reduce_mut(|queue_state| {
                                queue_state.queue.push_back(conf);
                            });
                        }
                        ServerMessage::RemoveQueueItem(finished_job) => {
                            // TODO Put the finished_job into another hook to enable viewing it somewhere else.
                            finished_job_dispatch.reduce_mut(|finished_job_state| {
                                finished_job_state.jobs.push(finished_job.clone());
                            });
                            queue_state_dispatch.reduce_mut(|queue_state| {
                                if queue_state.queue.pop_front().is_none() {
                                    log!("Error: Queue out of sync!");
                                }
                            });
                        }
                        _ => {
                            log!("Error: Unexpected websocket message received!");
                        }
                    }
                }
                //log!("Done!");
            });
        },
        (),
    );
    html! {
        <span class="position-absolute top-0 start-100 translate-middle badge rounded-pill bg-danger">
            {"CONNECTED"}
            <span class="visually-hidden">{"websocket status"}</span>
        </span>
    }
}
