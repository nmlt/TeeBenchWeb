use futures::{SinkExt, StreamExt};
use gloo_console::log;
use gloo_net::websocket::{futures::WebSocket, Message};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yewdux::prelude::*;
use std::collections::VecDeque;

use common::data_types::{
    //     Algorithm,
    //     Dataset,
    //     ExperimentType,
    //     Parameter,
    //     Platform,
    ProfilingConfiguration,
    QueueMessage,
};

#[derive(Debug, PartialEq, Properties)]
struct QueueItemProps {
    config: ProfilingConfiguration,
    running: bool,
}

#[function_component]
fn QueueItem(QueueItemProps{ config, running }: &QueueItemProps) -> Html {
    let spinner = if *running {
        html! {
            <div class="spinner-border" role="status">
                <span class="visually-hidden">{"Running..."}</span>
            </div>
        }
    } else {
        html! {
            <div class="m-5">
                <span class="visually-hidden">{"Waiting..."}</span>
            </div>
        }
    };
    html! {
        <li class="list-group-item">
            {spinner}
            <div>{format!("Profiling job with {config:?}")}</div>
        </li>
    }
}

// #[derive(Debug, PartialEq, Properties)]
// struct QueueListProps {
//     queue_state: QueueState,
// }

// #[function_component]
// fn QueueList(QueueListProps { queue_state }: &QueueListProps) -> Html {

//     html! {
//         <ul class="list-group">
//             {list_items_html}
//         </ul>
//     }
// }

#[derive(Debug, Clone, Default, PartialEq, Store)]
struct QueueState {
    queue: VecDeque<ProfilingConfiguration>,
}

#[function_component]
pub fn Queue() -> Html {
    let ws = match WebSocket::open("ws://localhost:3000/api/queue") {
        // `ws://` is required, otherwise there's an error.
        Ok(ws) => ws,
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
        log!("sending first...");
        write
            .send(Message::Bytes(serde_json::to_vec(&QueueMessage::RequestQueue).unwrap()))
            .await
            .unwrap();
        log!("Done!\nAwait-ing answer...");
        let dispatch = Dispatch::<QueueState>::new();
        while let Some(Ok(Message::Bytes(msg))) = read.next().await {
            let msg = serde_json::from_slice(&msg).unwrap();
            log!(format!("Got msg {msg:?}"));
            match msg {
                QueueMessage::AddQueueItem(conf) => {
                    dispatch.reduce_mut(|queue_state| {
                        queue_state.queue.push_back(conf);
                    });
                }
                QueueMessage::RemoveQueueItem(finished_job) => {
                    // TODO Put the finished_job into another hook to enable viewing it somewhere else.
                }
                _ => {
                    log!("Error: Unexpected websocket message received!");
                }
            }
        }
        log!("Done!");
    });

    let (queue_store, _dispatch) = use_store::<QueueState>();
    let queue: Vec<Html> = queue_store.queue.iter().map(|c| html! { <QueueItem config={c.clone()} running={false} /> }).collect();
    html! {
        <div class="text-white">
            <h3>{"Queue"}</h3>
            <ul class="list-group">
                {for queue}
            </ul>
        </div>
    }
}
