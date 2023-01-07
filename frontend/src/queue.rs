use futures::{SinkExt, StreamExt};
use gloo_console::log;
use gloo_net::websocket::{futures::WebSocket, Message};
use std::collections::VecDeque;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yewdux::prelude::*;

use common::data_types::{
    //     Algorithm,
    //     Dataset,
    //     ExperimentType,
    //     Parameter,
    //     Platform,
    ProfilingConfiguration,
    QueueMessage,
};

use crate::job_results_view::FinishedJobState;

#[derive(Debug, PartialEq, Properties)]
struct QueueItemProps {
    config: ProfilingConfiguration,
    running: bool,
}

#[function_component]
fn QueueItem(QueueItemProps { config, running }: &QueueItemProps) -> Html {
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
        <li class="list-group-item" title={format!("{config}")}>
            {spinner}
            //{"Submitted at: "}<span class="fw-bold">{format!("{} ", submitted.format(time_format).unwrap())}</span>
            {" "}
            <span class="badge text-bg-primary">{format!("{:?}", config.algorithm)}</span>
            <span>{" running... "}</span>
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
                        finished_job_state.jobs.push(finished_job);
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

    let (queue_store, _dispatch) = use_store::<QueueState>();
    let queue: Vec<Html> = queue_store
        .queue
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let running = if i == 0 { true } else { false };
            html! { <QueueItem config={c.clone()} running={running} /> }
        })
        .collect();
    html! {
        // <div class="text-white">
        //     <h3 class="fs-5">{"Queue"}</h3>
        //     <ul class="list-group">
        <>
            {for queue}
        </>
        //     </ul>
        // </div>
    }
}
