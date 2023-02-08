use yew::prelude::*;
use yew_router::components::Redirect;
use yewdux::prelude::*;

use crate::{chartjs::Chart, commits::CommitState, modal::Modal, navigation::Navigation, Route};
use common::data_types::{Finding, FindingStyle, Report};

#[derive(Debug, PartialEq, Properties)]
pub struct FindingCardColumnProps {
    finding: Finding,
}

#[function_component]
pub fn FindingCardColumn(FindingCardColumnProps { finding }: &FindingCardColumnProps) -> Html {
    let class_list = match finding.style {
        FindingStyle::Neutral => "background-color: #FFFFFF;",
        FindingStyle::Good => "background-color: #77DD77;",
        FindingStyle::SoSo => "background-color: #FDE26C;",
        FindingStyle::Bad => "background-color: #FF6961;",
    };
    html! {
        <div class="col-lg-2 col-sm-4">
            <div class="card my-4" style={class_list}>
                <div class="card-body">
                    <h5 class="card-text">{finding.title.clone()}</h5>
                    <h5 class="card-title">{finding.message.clone()}</h5>
                </div>
            </div>
        </div>
    }
}

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
        None => "MWAY", // TODO Make this the latest commit.
    };
    let Some(commit) = commit_store.commits.iter().filter(|c| &c.title == current).next() else {
        let titles = commit_store.commits.iter().map(|c| c.title.clone()).collect::<Vec<String>>();
        return html! {
            //<Redirect<Route> to={Route::NotFound} />
            {for titles}
        }
    };
    let findings = vec![
        Finding::new("Performance Difference", "+ 3.6 %", FindingStyle::Good),
        Finding::new("Phase 1: Partition", "180/191 (+0)", FindingStyle::SoSo),
        Finding::new("Phase 2: Join", "11/191 (-4)", FindingStyle::Good),
        Finding::new("EPC Paging", "- 0.4 %", FindingStyle::Good),
    ];
    let findings = findings.iter().map(|f| {
        html! {
            <FindingCardColumn finding={f.clone()} />
        }
    });
    let reports = vec![commit.report.clone().unwrap()];
    let reports = reports.iter().map(|r| {
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
