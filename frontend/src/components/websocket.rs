use yew::prelude::*;
use yewdux::prelude::*;
use std::collections::VecDeque;

use common::data_types::QueueMessage;
use futures::{SinkExt, StreamExt};
use gloo_console::log;
use gloo_net::websocket::{futures::WebSocket, Message};
use wasm_bindgen_futures::spawn_local;

#[derive(Debug, Clone, Default, PartialEq, Store)]
pub struct WebsocketState {
    pub outgoing_queue: VecDeque<QueueMessage>,
    pub connected: bool,
}

#[function_component]
pub fn Websocket() -> Html {
    let (store, dispatch) = use_store::<WebsocketState>();
    use_effect_with_deps(
        move |_| {
            let dispatch = dispatch.clone();
            // TODO Change address to 'api/ws'.
            // TODO Find a way for the frontend to get the websites actual address (not localhost)
            let ws = match WebSocket::open("ws://localhost:3000/api/queue") {
                // `ws://` is required, otherwise there's an error.
                Ok(ws) => {
                    dispatch.reduce_mut(|store| store.connected = true);
                    ws
                }
                Err(e) => {
                    log!(format!("Error opening websocket: {:?}", e));
                    panic!();
                }
            };
            // Send requestqueue
            // receive queue
            // loop: listen for remove queue
            let (mut write, mut read) = ws.split();

            spawn_local(async move {
                use crate::job_results_view::FinishedJobState;
                use crate::queue::QueueState;
                use common::data_types::QueueMessage;

                //log!("sending first...");
                write
                    .send(Message::Bytes(
                        serde_json::to_vec(&QueueMessage::RequestQueue).unwrap(),
                    ))
                    .await
                    .unwrap();
                //log!("Done!\nAwait-ing answer...");
                let queue_state_dispatch = Dispatch::<QueueState>::new();
                let finished_job_dispatch = Dispatch::<FinishedJobState>::new();
                while let Some(Ok(Message::Bytes(msg))) = read.next().await {
                    let msg = serde_json::from_slice(&msg).unwrap();
                    log!(format!("Got msg {msg:?}"));
                    match msg {
                        QueueMessage::AddQueueItem(conf) => {
                            queue_state_dispatch.reduce_mut(|queue_state| {
                                queue_state.queue.push_back(conf);
                            });
                        }
                        QueueMessage::RemoveQueueItem(finished_job) => {
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
        store,
    );
    html! {
        <span class="position-absolute top-0 start-100 translate-middle badge rounded-pill bg-danger">
            {"99+"}
            <span class="visually-hidden">{"websocket status"}</span>
        </span>
    }
}
