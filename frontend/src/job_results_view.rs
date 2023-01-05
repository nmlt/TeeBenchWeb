use serde::{Deserialize, Serialize};
use time::macros::format_description;
use yew::prelude::*;
use yewdux::prelude::*;

use common::data_types::{Job, Report, Finding};

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
                    content_dispatch.set_callback(move |_| {
                        let result = result.clone();
                        let (report,findings) = match result {
                            Ok(r) => (r.report, r.findings),
                            Err(_) => (Report::default(), HashSet::new()),
                        };
                        let findings = findings.iter().map(|f| {
                            match f {
                                Finding::SevereEpcPaging => html! {
                                    <div class="col-sm-2">
                                        <div class="card my-4">
                                            <div class="card-body">
                                                <h5 class="card-text">{"SevereEpcPaging"}</h5>
                                                <h5 class="card-title">{"+ 3.6 %"}</h5>
                                            </div>
                                        </div>
                                    </div>
                                },
                                Finding::MaxThroughput => html! {
                                     <div class="col-sm-2">
                                        <div class="card my-4">
                                            <div class="card-body">
                                                <h5 class="card-text">{"MaxThroughput"}</h5>
                                                <h5 class="card-title">{"+ 3.6 %"}</h5>
                                            </div>
                                        </div>
                                    </div>
                                },
                                Finding::CpuLogicalCores => html! {
                                     <div class="col-sm-2">
                                        <div class="card my-4">
                                            <div class="card-body">
                                                <h5 class="card-text">{"CpuLogicalCores"}</h5>
                                                <h5 class="card-title">{"+ 3.6 %"}</h5>
                                            </div>
                                        </div>
                                    </div>
                                },
                                Finding::CpuPhysicalCores => html! {
                                     <div class="col-sm-2">
                                        <div class="card my-4">
                                            <div class="card-body">
                                                <h5 class="card-text">{"CpuPhysicalCores"}</h5>
                                                <h5 class="card-title">{"+ 3.6 %"}</h5>
                                            </div>
                                        </div>
                                    </div>
                                },
                                Finding::SgxMaxCores => html! {
                                     <div class="col-sm-2">
                                        <div class="card my-4">
                                            <div class="card-body">
                                                <h5 class="card-text">{"SgxMaxCores"}</h5>
                                                <h5 class="card-title">{"+ 3.6 %"}</h5>
                                            </div>
                                        </div>
                                    </div>
                                },
                                Finding::NativeMaxCores => html! {
                                     <div class="col-sm-2">
                                        <div class="card my-4">
                                            <div class="card-body">
                                                <h5 class="card-text">{"NativeMaxCores"}</h5>
                                                <h5 class="card-title">{"+ 3.6 %"}</h5>
                                            </div>
                                        </div>
                                    </div>
                                }
                            }
                        });
                        ModalContent {
                            content: html! {
                                <div class="modal-content">
                                    <div class="modal-header">
                                        <h5 class="modal-title">{"Job Result"}</h5> // TODO Add a proper title.
                                        <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                                    </div>
                                    <div class="modal-body">
                                        <Chart report={report}/>
                                    </div>
                                    <div class="row" style="padding:20px">
                                        {for findings}
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
    use common::data_types::{
        Algorithm, Dataset, ExperimentType, Parameter, Platform, ProfilingConfiguration, Report,
        Measurement, ReportWithFindings, Finding
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
        submitted: time::OffsetDateTime::now_utc(),
        runtime: time::Duration::new(5, 0),
        result: Ok(ReportWithFindings{report: Report::default(), findings: HashSet::from([Finding::SevereEpcPaging])}),
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
        submitted: time::OffsetDateTime::now_utc(),
        runtime: time::Duration::new(180, 0),
        result: Ok(ReportWithFindings{
                        report: Report::ScalabilityNativeSgxExample,
                        findings: HashSet::from([Finding::MaxThroughput, Finding::CpuLogicalCores])})
    };
    html! {
        <ul class="list-group">
            {for jobs}
            <JobResult job={test_j} />
            <JobResult job={test_scalability} />
        </ul>
    }
}
