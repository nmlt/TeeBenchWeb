use serde::{Deserialize, Serialize};
use time::macros::format_description;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::queue::Queue;

use common::data_types::{FindingStyle, Job, Report};

use crate::chartjs::Chart;
use crate::modal::ModalContent;
use std::collections::HashSet;

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
                    let move_content_dispatch = content_dispatch.clone();
                    content_dispatch.set_callback(move |_| {
                        let content_dispatch = move_content_dispatch.clone();
                        let result = result.clone();
                        let (report,findings) = match result {
                            Ok(r) => (r.report, r.findings),
                            Err(_) => (Report::default(), HashSet::new()),
                        };
                        let findings = findings.iter().map(|f| {
                            let f = f.clone();
                            match f.style {
                                FindingStyle::Neutral => html! {
                                    <div class="col-sm-3">
                                        <div class="card my-4" style="background-color: #FFFFFF;">
                                            <div class="card-body">
                                                <h5 class="card-text">{f.title}</h5>
                                                <h5 class="card-title">{f.message}</h5>
                                            </div>
                                        </div>
                                    </div>
                                },
                                FindingStyle::Good => html! {
                                    <div class="col-sm-3">
                                        <div class="card my-4" style="background-color: #77DD77;">
                                            <div class="card-body">
                                                <h5 class="card-text">{f.title}</h5>
                                                <h5 class="card-title">{f.message}</h5>
                                            </div>
                                        </div>
                                    </div>
                                },
                                FindingStyle::SoSo => html! {
                                    <div class="col-sm-3">
                                        <div class="card my-4" style="background-color: #FDE26C;">
                                            <div class="card-body">
                                                <h5 class="card-text">{f.title}</h5>
                                                <h5 class="card-title">{f.message}</h5>
                                            </div>
                                        </div>
                                    </div>
                                },
                                FindingStyle::Bad => html! {
                                    <div class="col-sm-3">
                                        <div class="card my-4" style="background-color: #FF6961;">
                                            <div class="card-body">
                                                <h5 class="card-text">{f.title}</h5>
                                                <h5 class="card-title">{f.message}</h5>
                                            </div>
                                        </div>
                                    </div>
                                }
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
                                        <h5 class="modal-title">{"Job Result"}</h5> // TODO Add a proper title.
                                        <button type="button" class="btn-close" onclick={destroy_onclick.clone()} data-bs-dismiss="modal" aria-label="Close"></button>
                                    </div>
                                    <div class="modal-body">
                                        <Chart report={report}/>
                                    </div>
                                    <div class="modal-header">
                                        <h5 class="modal-title">{"Analyser Findings"}</h5> // TODO Add a proper title.
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
    use common::data_types::{
        Algorithm, Dataset, ExperimentType, Finding, Measurement, Parameter, Platform,
        ProfilingConfiguration, Report, ReportWithFindings,
    };
    let test_j = Job::Finished {
        config: ProfilingConfiguration {
            algorithm: HashSet::from([Algorithm::Cht]),
            experiment_type: ExperimentType::EpcPaging,
            parameter: Parameter::Threads,
            measurement: Measurement::Throughput,
            min: 3,
            max: 3,
            step: 3,
            dataset: Dataset::CacheExceed,
            platform: Platform::Sgx,
            sort_data: true,
        },
        submitted: time::OffsetDateTime::now_utc() - time::Duration::new(4000, 0),
        runtime: time::Duration::new(5, 0),
        result: Ok(ReportWithFindings {
            report: Report::EpcCht,
            findings: HashSet::from([
                Finding {
                    title: "MaxThroughput".to_owned(),
                    message: "64.22 M".to_owned(),
                    style: FindingStyle::Neutral,
                },
                Finding {
                    title: "Severe EPC Paging".to_owned(),
                    message: "Max EPC misses: 1870652".to_owned(),
                    style: FindingStyle::Bad,
                },
                Finding {
                    title: "Low throughput".to_owned(),
                    message: "Lowest throughput [M rec/s]: 0.97".to_owned(),
                    style: FindingStyle::Bad
                }]),
        }),
    };
    let test_scalability = Job::Finished {
        config: ProfilingConfiguration {
            algorithm: HashSet::from([Algorithm::Rho]),
            experiment_type: ExperimentType::EpcPaging,
            parameter: Parameter::Threads,
            measurement: Measurement::Throughput,
            min: 1,
            max: 8,
            step: 1,
            dataset: Dataset::CacheFit,
            platform: Platform::Sgx,
            sort_data: false,
        },
        submitted: time::OffsetDateTime::now_utc() - time::Duration::new(3600, 0),
        runtime: time::Duration::new(180, 0),
        result: Ok(ReportWithFindings {
            report: Report::ScalabilityNativeSgxExample,
            findings: HashSet::from([
                Finding {
                    title: "CPU Logical Cores".to_owned(),
                    message: "8".to_owned(),
                    style: FindingStyle::Neutral,
                },
                Finding {
                    title: "Optimal CPU cores Native".to_owned(),
                    message: "6".to_owned(),
                    style: FindingStyle::SoSo,
                },
                Finding {
                    title: "Optimal CPU cores SGX".to_owned(),
                    message: "2".to_owned(),
                    style: FindingStyle::Bad,
                },
            ]),
        }),
    };
    html! {
        <ul class="list-group">
            <JobResult job={test_j} />
            <JobResult job={test_scalability} />
            {for jobs}
            <Queue />
        </ul>
    }
}
