use yew::prelude::*;
use yew_router::prelude::*;

mod chartjs;
mod commits;
mod job_results_view;
mod modal;
mod navigation;
mod perf_report;
mod profiling;
mod queue;

use crate::commits::Commits;
use crate::perf_report::PerfReport;
use crate::profiling::Profiling;

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/commits")]
    Commits,
    #[at("/profiling")]
    Profiling,
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
        Route::PerfReport { commit } => html! {
            <PerfReport {commit}/>
        },
        Route::NotFound => html! { <main><h1>{ "404" }</h1><p>{"not found in yew app"}</p></main> },
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
    yew::Renderer::<App>::new().render();
}
