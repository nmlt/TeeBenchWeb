use std::collections::VecDeque;
use time::macros::format_description;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::components::tag::Tag;

use common::data_types::{Job, JobConfig, JobIdType};

#[derive(Debug, PartialEq, Properties)]
struct QueueItemProps {
    job: Job,
    running: bool,
}

#[function_component]
fn QueueItem(QueueItemProps { job, running }: &QueueItemProps) -> Html {
    let time_format = format_description!("[hour]:[minute]");
    let (spinner, desc) = if *running {
        (
            html! {
                <div class="spinner-border" role="status">
                    <span class="visually-hidden">{"Running..."}</span>
                </div>
            },
            "running...",
        )
    } else {
        (
            html! {
                <div class="spinner-border opacity-0">
                    <span class="visually-hidden">{"Waiting..."}</span>
                </div>
            },
            "waiting...",
        )
    };
    let algs: Vec<_> = job
        .config
        .algorithms(None)
        .into_iter()
        .map(|a| html! { <Tag text={a} /> })
        .collect();
    html! {
        <li class="list-group-item" title={format!("{}", job.config)}>
            <div class="">
                <span class="px-1">{spinner}</span>
                <span class="px-1">{"Submitted at: "}<span class="fw-bold">{format!("{} ", job.submitted.format(time_format).unwrap())}</span></span>
                <span class="px-1">{for algs}</span>
                <span class="px-1">{desc}</span>
                <span class="float-end"><RemoveJobButton id={job.id} /></span>
            </div>
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
    pub queue: VecDeque<Job>,
}

impl QueueState {
    pub fn new(jobs: Vec<Job>) -> Self {
        let queue: VecDeque<Job> = jobs.into();
        QueueState { queue }
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct QueueProps {
    pub filter_by: Option<JobConfig>,
}

#[function_component]
pub fn Queue(QueueProps { filter_by }: &QueueProps) -> Html {
    let (queue_store, _queue_dispatch) = use_store::<QueueState>();
    let queue: Vec<Html> = queue_store
        .queue
        .iter()
        .filter(|j| {
            if let Some(filter_by) = filter_by {
                std::mem::discriminant(filter_by) == std::mem::discriminant(&j.config)
            } else {
                true
            }
        })
        .enumerate()
        .map(|(i, j)| {
            let running = if i == 0 { true } else { false };
            html! { <QueueItem job={j.clone()} running={running} /> }
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
    let (queue_store, queue_dispatch) = use_store::<QueueState>();
    let empty = queue_store.queue.is_empty();
    let onclick = {
        let websocket_store = use_store_value::<WebsocketState>();
        queue_dispatch.reduce_mut_callback(move |s| {
            websocket_store.send(ClientMessage::RemoveAllJobs);
            s.queue.clear();
        })
    };
    html! {
        <button class="btn btn-danger" disabled={empty} {onclick}>{"Clear Queue"}</button>
    }
}

#[derive(PartialEq, Properties)]
pub struct RemoveJobButtonProps {
    pub id: JobIdType,
}

#[function_component]
pub fn RemoveJobButton(RemoveJobButtonProps { id }: &RemoveJobButtonProps) -> Html {
    let (queue_store, queue_dispatch) = use_store::<QueueState>();
    let empty = queue_store.queue.is_empty();
    let onclick = {
        let websocket_store = use_store_value::<WebsocketState>();
        let id = id.clone();
        queue_dispatch.reduce_mut_callback(move |s| {
            websocket_store.send(ClientMessage::RemoveJob(id));
            if let Some(pos) = s.queue.iter().position(|j| j.id == id) {
                s.queue.remove(pos);
            }
        })
    };
    html! {
        <button class="btn btn-danger" disabled={empty} {onclick}>
            <i class="bi-trash"></i>
        </button>
    }
}
