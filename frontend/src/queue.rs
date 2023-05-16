use gloo_console::log;
use gloo_net::http::{Method, Request};
use std::collections::VecDeque;
use yew::platform::spawn_local;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::components::tag::Tag;

use common::data_types::{Job, JobConfig, ProfilingConfiguration};

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
    let algs: Vec<_> = config
        .algorithms
        .iter()
        .map(|a| a.to_string())
        .map(|a| html! { <Tag text={a} /> })
        .collect();
    html! {
        <li class="list-group-item" title={format!("{config}")}>
            {spinner}
            //{"Submitted at: "}<span class="fw-bold">{format!("{} ", submitted.format(time_format).unwrap())}</span>
            {" "}
            {for algs}
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

/// This holds the queue. It gets filled by the websocket spawned in the crate::app function.
#[derive(Debug, Clone, Default, PartialEq, Store)]
pub struct QueueState {
    pub queue: VecDeque<ProfilingConfiguration>,
}

impl QueueState {
    pub fn new(jobs: Vec<Job>) -> Self {
        let queue: VecDeque<ProfilingConfiguration> = jobs
            .into_iter()
            .map(|j| {
                if let Job {
                    config: JobConfig::Profiling(c),
                    ..
                } = j
                {
                    return Some(c);
                }
                // Nothing to display, other kind of jobs are shown in the Operator (commits.rs) view.
                None
            })
            .flatten()
            .collect();
        QueueState { queue }
    }
}

#[function_component]
pub fn Queue() -> Html {
    let (queue_store, queue_dispatch) = use_store::<QueueState>();
    use_effect_with_deps(
        move |_| {
            let queue_dispatch = queue_dispatch.clone();
            spawn_local(async move {
                let resp: Result<Vec<Job>, _> = Request::get("/api/queue")
                    .method(Method::GET)
                    .send()
                    .await
                    .expect("Server didn't respond. Is it running?")
                    .json()
                    .await;
                match resp {
                    Ok(json) => {
                        log!(format!("got queue items: {json:?}"));
                        queue_dispatch.set(QueueState::new(json));
                    }
                    Err(e) => log!("Error getting queue json: ", e.to_string()),
                }
            });
        },
        (),
    );
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
            <ClearQueueButton />
        </>
        //     </ul>
        // </div>
    }
}

use crate::components::websocket::WebsocketState;
use common::data_types::ClientMessage;
#[function_component]
pub fn ClearQueueButton() -> Html {
    let queue_store = use_store_value::<QueueState>();
    let empty = queue_store.queue.is_empty();
    let onclick = {
        let websocket_store = use_store_value::<WebsocketState>();
        Callback::from(move |_| {
            let websocket_store = websocket_store.clone();
            websocket_store.send(ClientMessage::RemoveAllJobs);
        })
    };
    html! {
        <button class="btn btn-danger" disabled={empty} {onclick}>{"Clear Queue"}</button>
    }
}
