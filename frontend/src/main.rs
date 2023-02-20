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

use crate::commits::{CommitState, Commits};
use crate::perf_report::PerfReport;
use crate::profiling::Profiling;

use common::data_types::{Commit, Report};
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
    let default_commits = vec![
        Commit::new(
            "RHT".to_owned(),
            "JOIN".to_owned(),
            OffsetDateTime::now_utc(),
            include_str!("../deps/radix_join.c").to_owned(),
            vec![Report::Epc { findings: vec![] }],
        ),
        Commit::new(
            "CHT".to_owned(),
            "JOIN".to_owned(),
            OffsetDateTime::now_utc(),
            "blah".to_owned(),
            vec![Report::Scalability { findings: vec![] }],
        ),
        Commit::new(
            "PHT".to_owned(),
            "JOIN".to_owned(),
            OffsetDateTime::now_utc(),
            "blah".to_owned(),
            vec![Report::ScalabilityNativeSgxExample { findings: vec![] }],
        ),
        Commit::new(
            "MWAY".to_owned(),
            "JOIN".to_owned(),
            OffsetDateTime::now_utc(),
            "blah".to_owned(),
            vec![Report::Throughput { findings: vec![] }],
        ),
    ];
    Dispatch::<CommitState>::new().set(CommitState {
        commits: default_commits,
    });
    yew::Renderer::<App>::new().render();
}
