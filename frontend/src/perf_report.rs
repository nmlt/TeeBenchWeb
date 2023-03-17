use yew::prelude::*;
use yewdux::prelude::*;

use crate::{
    chartjs::Chart,
    commits::{CommitState, CommitStatus},
    components::finding::FindingCardColumn,
    modal::Modal,
    navigation::Navigation,
};
use common::data_types::{JobResult, Report};

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
        Some(c) => c,
        None => match commit_store.0.iter().last() {
            Some(cs) => &cs.commit.title,
            None => {
                return html! {
                    <h1>{"No operators! Upload some in the Operator tab."}</h1> // TODO Would be nice to provide a link to the Operator tab.
                };
            }
        },
    };
    let Some(commit) = commit_store.0.iter().filter(|cs| &cs.commit.title == current).next().map(|cs| cs.commit.clone()) else {
        return html! {
            <h1>{format!("Error getting commit with title {current:?}!")}</h1>
        }
    };
    let findings = commit.findings.iter().map(|f| {
        html! {
            <FindingCardColumn finding={f.clone()} />
        }
    });
    let reports = commit.reports.iter().map(|r: &JobResult| {
        let r = match r {
            JobResult::Exp(Ok(r)) => r,
            JobResult::Exp(Err(_)) => {
                return html! {
                    "Error while running experiment!"
                }
            }
            JobResult::Compile(_) => {
                return html! {
                    "Cannot show compile jobs as chart!"
                }
            }
        };
        let chart = html! {
            <Chart report={r.clone()} />
        };
        html! {
            <CardChartColumn chart={chart} />
        }
    });
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
                                {for reports}
                            </div>
                        </div>
                    </main>
                </div>
            </div>
            <Modal />
        </div>
    }
}
