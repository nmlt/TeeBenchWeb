use yew::prelude::*;
use yewdux::prelude::*;

use crate::{
    chartjs::Chart, components::finding::FindingCardColumn, modal::Modal, navigation::Navigation,
};
use common::commit::CommitState;
use common::data_types::JobResult;

#[derive(Debug, PartialEq, Properties)]
pub struct CardChartColumnProps {
    chart: Html,
}

#[function_component]
pub fn CardChartColumn(CardChartColumnProps { chart }: &CardChartColumnProps) -> Html {
    html! {
        <div class="col">
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
    pub commit: Option<String>,
}

#[function_component]
pub fn PerfReport(PerfReportProps { commit: current }: &PerfReportProps) -> Html {
    let commit_store = use_store_value::<CommitState>();
    let current = match current {
        Some(title) => match commit_store.get_by_title(&title).first() {
            // TODO If there are multiple commits with the same title, add another route "perf_report/title/<number>" and handle the vector returned here accordingly.
            Some(c) => c.id,
            None => {
                return html! {
                    <h1>{format!("Error getting commit with title {title:?}!")}</h1>
                }
            }
        },
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
        charts = report
            .charts
            .iter()
            .map(|exp_chart| {
                let exp_chart = exp_chart.clone();
                let chart = html! {
                    <Chart exp_chart={exp_chart.clone()} />
                };
                html! {
                    <CardChartColumn chart={chart} />
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
        let msg = if commit.perf_report_running {
            "Waiting for performance report generation..."
        } else {
            "To start generating a performance report switch to the Operator tab."
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
                            <h2>{format!("Performance Report for {}", commit.title)}</h2>
                            <div class="row">
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
