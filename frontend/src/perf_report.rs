use yew::prelude::*;
use yewdux::prelude::*;

use crate::{chartjs::Chart, commits::CommitState, modal::Modal, navigation::Navigation};
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
        <div class="col-xl-2 col-lg-4">
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
        None => &commit_store.commits.iter().last().unwrap().title, // Latest commit
    };
    if commit_store.commits.len() == 0 {
        return html! {
            <h1>{"No data! Run some experiments."}</h1> // TODO Would be nice to provide a link to the Profiling tab.
        };
    }
    // TODO THE QUESTION IS: What do I show, depending on how we store experiment results?
    // - Simply all graphs that were generated in the profiling tab. But how do I select which algorithm? If the user goes to perf_report/MWAY do I show the comparison of MWAY to RHO _and_ MWAY to CHT? Seems like I need a database and precise queries.
    // - For now, I'm not splitting comparison experiments. So each experiment that contains eg. CHT will get added to CHT algorithm commit state, and displayed here.
    let Some(commit) = commit_store.commits.iter().filter(|c| &c.title == current).next() else {
        return html! {
            <h1>{format!("Error getting commit with title {current:?}!")}</h1>
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
    let reports = commit.reports.iter().map(|r: &Report| {
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
