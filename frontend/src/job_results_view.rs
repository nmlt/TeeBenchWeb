use gloo_console::log;
use serde::{Deserialize, Serialize};
use time::macros::format_description;
use yew::prelude::*;
use yewdux::prelude::*;

use common::data_types::{Algorithm, Job, JobConfig, JobResult, JobStatus, Report};

use crate::chart::Chart;
use crate::components::{finding::FindingCardColumn, tag::Tag};
use crate::modal::ModalContent;
use crate::queue::Queue;
use common::commit::CommitState;

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
                c.algorithms
                    .iter()
                    .map(|a| match a {
                        Algorithm::Commit(id) => {
                            let title = commit_store.get_by_id(id).map(|c| c.title.clone()).unwrap_or_else(|| {
                                log!(format!("Could not get commit with id {id}. Maybe the render function was quicker than the REST request? (Ignore this message if the Algorithm/Operator labels look okay.)"));
                                "Latest Operator (not yet loaded, check the connection)".to_string()
                            });
                            html! { <Tag text={title} /> }
                        }
                        a => html! {
                            <Tag text={a.to_string()} />
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
                        // TODO Make destroying part of the modal.rs file, don't do it here.
                        let destroy_onclick = content_dispatch.set_callback(|_| {
                            ModalContent {
                                content: html! {
                                    <>
                                    </>
                                }
                            }
                        });
                        let charts = report.charts.iter().map(|exp_chart| {
                            html! {
                                <Chart exp_chart={exp_chart.clone()}/>
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
                                        {for charts}
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
                    <span>{format!(" took {runtime:.1} ")}</span>
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
