use gloo_console::log;
use yew::prelude::*;
use yew_router::prelude::Redirect;
use yewdux::prelude::*;

use crate::{
    chart::Chart, components::finding::FindingCardColumn, modal::Modal, navigation::Navigation,
    Route,
};
use common::commit::{CommitState, PerfReportStatus};
use common::data_types::JobResult;

#[derive(Debug, PartialEq, Properties)]
pub struct CardChartColumnProps {
    id_class: String,
    chart: Html,
}

#[function_component]
pub fn CardChartColumn(CardChartColumnProps { id_class, chart }: &CardChartColumnProps) -> Html {
    let classes = classes!("col", id_class);
    html! {
        <div classes={classes}>
            <div class="card my-4">
                <div class="card-body ratio ratio-16x9">
                    {chart.clone()}
                </div>
            </div>
        </div>
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct PerfReportProps {
    pub name: Option<String>,
    pub instance: Option<usize>,
}

#[function_component]
pub fn PerfReport(PerfReportProps { name, instance }: &PerfReportProps) -> Html {
    let commit_store = use_store_value::<CommitState>();
    log!("Running PerfReport component!");
    let current = match name {
        Some(name) => {
            let candidates = commit_store.get_by_name(name);
            if candidates.len() > 1 {
                if let Some(instance) = instance {
                    candidates[*instance].id
                } else {
                    return html! {
                        <Redirect<Route> to={Route::perf_report_double(name.clone(), 0)} />
                    };
                }
            } else if candidates.len() == 1 {
                candidates.first().unwrap().id
            } else {
                return html! {
                    <h1>{format!("Error getting commit with name {name:?}!")}</h1>
                };
            }
        }
        None => match commit_store.get_latest() {
            Some(c) => c.id,
            None => {
                return html! {
                    <h1>{"No operators! Upload some in the Operator tab."}</h1> // TODO Would be nice to provide a link to the Operator tab.
                };
            }
        },
    };
    let Some(commit) = commit_store.get_by_id(&current) else {
        return html! {
            <h1>{format!("Error getting commit with title {current:?}!")}</h1>
        }
    };
    let findings;
    let charts;
    if let Some(JobResult::Exp(Ok(ref report))) = commit.report {
        findings = report
            .findings
            .iter()
            .map(|f| {
                html! {
                    <FindingCardColumn finding={f.clone()} />
                }
            })
            .collect::<Vec<_>>();
        let id_classes = vec![
            "tbw-report-charts-throughput-cache-fit",
            "tbw-report-charts-throughput-cache-exceed",
            "tbw-report-charts-scalability-cache-fit",
            "tbw-report-charts-scalability-cache-exceed",
            "tbw-report-charts-epc-paging-latest-alg",
            "tbw-report-charts-epc-paging-baseline",
        ];
        charts = report
            .charts
            .iter()
            .zip(id_classes.iter())
            .map(|(exp_chart, id_class)| {
                let exp_chart = exp_chart.clone();
                let chart = html! {
                    <Chart exp_chart={exp_chart.clone()} />
                };
                html! {
                    <CardChartColumn id_class={id_class.to_owned()} chart={chart} />
                }
            })
            .collect::<Vec<_>>();
    } else if let Some(JobResult::Exp(Err(ref e))) = commit.report {
        findings = vec![];
        charts = vec![html! {
            <div class="alert alert-danger mx-2" role="alert">
                <p>{"There was an error generating the performance report."}</p>
                <p>{format!("Error:\n{e}")}</p>
            </div>
        }];
    } else {
        findings = vec![];
        let msg = match commit.perf_report_running {
            PerfReportStatus::None => {
                "To start generating a performance report switch to the Operator tab."
            }
            PerfReportStatus::Running(_) => "Waiting for performance report generation...",
            // TODO Check this somewhere further up, having this case here doesn't make sense.
            PerfReportStatus::Successful => unreachable!(),
            PerfReportStatus::Failed => {
                "Error creating this performance report. Log at the server logs!"
            }
        };
        charts = vec![html! {
            <div class="alert alert-info mx-2" role="alert">
                <p>{msg}</p>
            </div>
        }];
    }
    html! {
        <div class="container-fluid">
            <div class="row vh-100">
                <div class="col-12 col-sm-3 col-xl-2 px-sm-2 px-0 bg-dark d-flex sticky-top">
                    <Navigation active_nav_item={"PerfReport"} />
                </div>
                <div class="col d-flex flex-column h-sm-100">
                    <main class="row">
                        <div class="col pt-4">
                            <h2>{format!("Performance Report for {}", commit.get_title())}</h2>
                            <div class="row tbw-report-general-findings">
                                // Top row
                                {for findings}
                            </div>
                            <div class="row row-cols-2">
                                // Graph cards
                                {for charts}
                            </div>
                        </div>
                    </main>
                </div>
            </div>
            <Modal />
        </div>
    }
}
