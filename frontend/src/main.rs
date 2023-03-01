use yew::prelude::*;
use yew_router::prelude::*;

mod chartjs;
mod commits;
mod components;
mod job_results_view;
mod modal;
mod navigation;
mod perf_report;
mod profiling;
mod queue;

use crate::commits::Commits;
use crate::perf_report::PerfReport;
use crate::profiling::Profiling;

use common::data_types::Commit;
use time::OffsetDateTime;
use yewdux::prelude::Dispatch;

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/commits")]
    Commits,
    #[at("/profiling")]
    Profiling,
    #[at("/performance_report")]
    PerfReportLatest,
    #[at("/performance_report/:commit")]
    PerfReport { commit: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Redirect<Route> to={Route::Commits}/> },
        Route::Commits => html! {
            <Commits />
        },
        Route::Profiling => html! {
            <Profiling />
        },
        Route::PerfReportLatest => html! {
            <PerfReport commit={None::<String>} />
        },
        Route::PerfReport { commit } => html! {
            <PerfReport commit={Some(commit)} />
        },
        Route::NotFound => html! { <main><h1>{"404"}</h1><p>{"not found in yew app"}</p></main> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    use crate::commits::CommitState;

    use common::data_types::Report;
    let default_commits = vec![
        Commit::new(
            "RHT".to_owned(),
            "JOIN".to_owned(),
            OffsetDateTime::now_utc(),
            include_str!("../deps/radix_join.c").to_owned(),
            vec![Report::default()],
        ),
        Commit::new(
            "CHT".to_owned(),
            "JOIN".to_owned(),
            OffsetDateTime::now_utc(),
            "blah".to_owned(),
            vec![Report::default()],
        ),
        Commit::new(
            "PHT".to_owned(),
            "JOIN".to_owned(),
            OffsetDateTime::now_utc(),
            "blah".to_owned(),
            vec![Report::default()],
        ),
        Commit::new(
            "MWAY".to_owned(),
            "JOIN".to_owned(),
            OffsetDateTime::now_utc(),
            "blah".to_owned(),
            vec![Report::default()],
        ),
    ];
    Dispatch::<CommitState>::new().set(CommitState {
        commits: default_commits,
    });
    // use crate::job_results_view::FinishedJobState;
    // use common::data_types::{
    //     Algorithm, Dataset, ExperimentType, Finding, FindingStyle, Job, Measurement, Parameter,
    //     Platform, ProfilingConfiguration, Report, ReportWithFindings,
    // };
    // use std::collections::HashSet;
    // let default_job_results = vec![
    //     Job::Finished {
    //         config: ProfilingConfiguration {
    //             algorithm: HashSet::from([Algorithm::Cht]),
    //             experiment_type: ExperimentType::EpcPaging,
    //             parameter: Parameter::Threads,
    //             measurement: Measurement::Throughput,
    //             min: 3,
    //             max: 3,
    //             step: 3,
    //             dataset: HashSet::from([Dataset::CacheExceed]),
    //             platform: HashSet::from([Platform::Sgx]),
    //             sort_data: true,
    //         },
    //         submitted: time::OffsetDateTime::now_utc() - time::Duration::new(4000, 0),
    //         runtime: time::Duration::new(5, 0),
    //         result: Ok(ReportWithFindings {
    //             report: Report::EpcCht { findings: vec![] },
    //             findings: Vec::from([
    //                 Finding {
    //                     title: "MaxThroughput".to_owned(),
    //                     message: "64.22 M".to_owned(),
    //                     style: FindingStyle::Neutral,
    //                 },
    //                 Finding {
    //                     title: "Severe EPC Paging".to_owned(),
    //                     message: "Max EPC misses: 1870652".to_owned(),
    //                     style: FindingStyle::Bad,
    //                 },
    //                 Finding {
    //                     title: "Low throughput".to_owned(),
    //                     message: "Lowest throughput [M rec/s]: 0.97".to_owned(),
    //                     style: FindingStyle::Bad,
    //                 },
    //             ]),
    //         }),
    //     },
    //     Job::Finished {
    //         config: ProfilingConfiguration {
    //             algorithm: HashSet::from([Algorithm::Rho]),
    //             experiment_type: ExperimentType::EpcPaging,
    //             parameter: Parameter::Threads,
    //             measurement: Measurement::Throughput,
    //             min: 1,
    //             max: 8,
    //             step: 1,
    //             dataset: HashSet::from([Dataset::CacheFit]),
    //             platform: HashSet::from([Platform::Sgx]),
    //             sort_data: false,
    //         },
    //         submitted: time::OffsetDateTime::now_utc() - time::Duration::new(3600, 0),
    //         runtime: time::Duration::new(180, 0),
    //         result: Ok(ReportWithFindings {
    //             report: Report::ScalabilityNativeSgxExample { findings: vec![] },
    //             findings: Vec::from([
    //                 Finding {
    //                     title: "Throughput ratio (Native/SGX)".to_owned(),
    //                     message: "7x - 31x".to_owned(),
    //                     style: FindingStyle::Neutral,
    //                 },
    //                 Finding {
    //                     title: "Best CPU cores Native".to_owned(),
    //                     message: "6 / 8".to_owned(),
    //                     style: FindingStyle::SoSo,
    //                 },
    //                 Finding {
    //                     title: "Best CPU cores SGX".to_owned(),
    //                     message: "2 / 8".to_owned(),
    //                     style: FindingStyle::Bad,
    //                 },
    //                 Finding {
    //                     title: "CPU context-switch - High".to_owned(),
    //                     message: "SGX".to_owned(),
    //                     style: FindingStyle::Bad,
    //                 },
    //                 Finding {
    //                     title: "EPC Paging - Medium".to_owned(),
    //                     message: "SGX".to_owned(),
    //                     style: FindingStyle::SoSo,
    //                 },
    //             ]),
    //         }),
    //     },
    // ];
    // Dispatch::<FinishedJobState>::new().set(FinishedJobState {
    //     jobs: default_job_results,
    // });
    yew::Renderer::<App>::new().render();
}
