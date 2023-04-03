use serde::{Deserialize, Serialize};
use time::macros::format_description;
use yew::prelude::*;
use yewdux::prelude::*;

use common::data_types::{Algorithm, Job, JobConfig, JobResult, JobStatus, Report};

use crate::chartjs::Chart;
use crate::commits::CommitState;
use crate::components::finding::FindingCardColumn;
use crate::modal::ModalContent;
use crate::queue::Queue;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct JobResultViewProps {
    pub job: Job,
}

#[function_component]
pub fn JobResultView(JobResultViewProps { job }: &JobResultViewProps) -> Html {
    let (_content_store, content_dispatch) = use_store::<ModalContent>();
    let commit_store = use_store_value::<CommitState>();
    let time_format = format_description!("[hour]:[minute]");
    match &job.status {
        JobStatus::Waiting => html! { <span>{"Error!"}</span> },
        JobStatus::Done { runtime, result } => {
            let algs: Vec<_> = if let JobConfig::Profiling(c) = &job.config {
                c.algorithm
                    .iter()
                    .map(|a| match a {
                        Algorithm::Commit(id) => {
                            let c = commit_store.get_id(id).expect("Could not find commit!");
                            html! { <span class="badge text-bg-primary m-1">{c.title.clone()}</span> }
                        }
                        a => html! {
                            <span class="badge text-bg-primary m-1">{a.to_string()}</span>
                        }
                    })
                    .collect()
            } else {
                panic!("Can only display Profiling Jobs here!");
            };
            let result = match result {
                JobResult::Exp(r) => r,
                JobResult::Compile(_) => {
                    panic!("Cannot display compile results in job results view!")
                }
            };
            let result = if result.is_ok() {
                let result = result.clone();
                let onclick = {
                    let move_content_dispatch = content_dispatch.clone();
                    let algs = algs.clone();
                    content_dispatch.set_callback(move |_| {
                        let content_dispatch = move_content_dispatch.clone();
                        let result = result.clone();
                        let report = match result {
                            Ok(r) => r,
                            Err(_) => Report::default(),
                        };
                        let findings = report.findings.clone();
                        let findings = findings.iter().map(|f| {
                            let f = f.clone();
                            html! {
                                <FindingCardColumn finding={f} />
                            }
                        });
                        let destroy_onclick = content_dispatch.set_callback(|_| {
                            ModalContent {
                                content: html! {
                                    <>
                                    </>
                                }
                            }
                        });
                        ModalContent {
                            content: html! {
                                <div class="modal-content">
                                    <div class="modal-header">
                                        <h5 class="modal-title">{"Job Result for "}{for algs.clone()}</h5>
                                        <button type="button" class="btn-close" onclick={destroy_onclick.clone()} data-bs-dismiss="modal" aria-label="Close"></button>
                                    </div>
                                    <div class="modal-body">
                                        <Chart exp_chart={report.charts[0].clone()}/> // TODO Put the proper chart here
                                    </div>
                                    <div class="modal-header">
                                        <h5 class="modal-title">{"Analyser Findings"}</h5>
                                    </div>
                                    <div class="row" style="padding:20px">
                                        {for findings}
                                    </div>
                                    <div class="modal-footer">
                                        <button type="button" class="btn btn-secondary" onclick={destroy_onclick.clone()} data-bs-dismiss="modal">{"Close"}</button>
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
                <li class="list-group-item" title={format!("{}", job.config)}>
                    {"Submitted at: "}<span class="fw-bold">{format!("{} ", job.submitted.format(time_format).unwrap())}</span>
                    {for algs}
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
        html! { <JobResultView job={j.clone()} /> }
    });

    html! {
        <ul class="list-group">
            {for jobs}
            <Queue />
        </ul>
    }
}
