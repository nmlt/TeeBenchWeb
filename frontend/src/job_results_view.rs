use serde::{Deserialize, Serialize};
use time::macros::format_description;
use yew::prelude::*;
use yewdux::prelude::*;

use common::data_types::Job;

use crate::modal::ModalContent;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct JobResultProps {
    pub job: Job,
}

#[function_component]
pub fn JobResult(JobResultProps { job }: &JobResultProps) -> Html {
    let (_content_store, content_dispatch) = use_store::<ModalContent>();
    let time_format = format_description!("[hour]:[minute]");
    match job {
        Job::Running(_) => html! { <span>{"Error!"}</span> },
        Job::Finished {
            config,
            submitted,
            runtime,
            result,
        } => {
            let result = if result.is_ok() {
                let result = result.clone();
                let onclick = {
                    content_dispatch.set_callback(move |_| {
                        let result = result.clone();
                        ModalContent {
                            content: html! {
                                <div class="modal-content">
                                    <div class="modal-header">
                                        <h5 class="modal-title">{"Job Result"}</h5> // TODO Add a proper title.
                                        <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                                    </div>
                                    <div class="modal-body">
                                        <p>{format!("{result:?}")}</p>
                                    </div>
                                    <div class="modal-footer">
                                        <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{"Close"}</button>
                                    </div>
                                </div>
                            }
                        }
                    })
                };
                html! {<button class="btn btn-info" type="button" {onclick} data-bs-toggle="modal" data-bs-target="#mainModal">{"Results"}</button>}
            } else {
                html! {{"Error! No results."}}
            };
            html! {
                <li class="list-group-item" title={format!("{config}")}>
                    {"Submitted at: "}<span class="fw-bold">{format!("{} ", submitted.format(time_format).unwrap())}</span>
                    <span class="badge text-bg-primary">{format!("{:?}", config.algorithm)}</span>
                    <span>{format!(" took {runtime} ")}</span>
                    {result}
                </li>
            }
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Store)]
#[store(storage = "session")]
pub struct FinishedJobState {
    pub jobs: Vec<Job>,
}

#[function_component]
pub fn JobResultsView() -> Html {
    let (finished_job_store, _dispatch) = use_store::<FinishedJobState>();
    let jobs = finished_job_store.jobs.iter().map(|j| {
        html! { <JobResult job={j.clone()} /> }
    });
    html! {
        <ul class="list-group">
            {for jobs}
        </ul>
    }
}
