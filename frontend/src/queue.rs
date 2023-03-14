use std::collections::VecDeque;
use yew::prelude::*;
use yewdux::prelude::*;

use common::data_types::ProfilingConfiguration;

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
        .algorithm
        .iter()
        .map(|a| a.to_string())
        .map(|a| html! { <span class="badge text-bg-primary m-1">{a}</span> })
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

#[function_component]
pub fn Queue() -> Html {
    let queue_store = use_store_value::<QueueState>();
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
            websocket_store.send(ClientMessage::RequestClear);
        })
    };
    html! {
        <button class="btn btn-danger" disabled={empty} {onclick}>{"Clear Queue"}</button>
    }
}
