use serde::{Deserialize, Serialize};
use time::macros::format_description;
use wasm_bindgen_futures::spawn_local;
use gloo_timers::future::TimeoutFuture;
use yew::prelude::*;
use yewdux::prelude::*;

use common::data_types::Job;

use crate::modal::ModalContent;
use crate::chartjs::draw_chart;
use std::collections::HashSet;

#[function_component]
pub fn Chart() -> Html {
    spawn_local(async {
        // We have to wait until the canvas has been created.
        TimeoutFuture::new(100).await;
        draw_chart("chartjs-canvas");
    });
    html! {
        <div>
            <canvas id="chartjs-canvas"></canvas>
        </div>
    }
}

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
                                        <Chart />
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
    use common::data_types::{ProfilingConfiguration, Algorithm, ExperimentType, Parameter, Dataset, Platform, Report};
    let test_j = Job::Finished {
        config: ProfilingConfiguration {
            algorithm: HashSet::from([Algorithm::Cht]),
            experiment_type: ExperimentType::EpcPaging,
            parameter: Parameter::Threads,
            min: 3,
            max: 3,
            step: 3,
            dataset: Dataset::CacheExceed,
            platform: Platform::Sgx,
            sort_data: true,
        },
        submitted: time::OffsetDateTime::now_utc(),
        runtime: time::Duration::new(5, 0),
        result: Ok(Report::default()),
    };
    html! {
        <ul class="list-group">
            {for jobs}
            <JobResult job={test_j} />
        </ul>
    }
}
