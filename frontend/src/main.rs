use gloo_console::log;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::use_store_value;

mod chart;
mod commits;
mod components;
mod job_results_view;
mod js_bindings;
mod modal;
mod navigation;
mod perf_report;
mod profiling;
mod queue;

use crate::commits::Commits;
use crate::components::websocket::Websocket;
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

#[function_component]
fn App() -> Html {
    let websocket = if cfg!(feature = "static") {
        html! {}
    } else {
        html! {
            <Websocket />
        }
    };
    html! {
        <>
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
        {websocket}
        </>
    }
}

fn main() {
    use crate::job_results_view::FinishedJobState;
    use common::commit::CommitState;
    use common::hardcoded::{hardcoded_profiling_jobs, predefined_commit};
    use yewdux::prelude::Dispatch;

    let finished_job_dispatch = Dispatch::<FinishedJobState>::new();
    if cfg!(feature = "static") {
        let default_commits = vec![predefined_commit()];
        Dispatch::<CommitState>::new().set(CommitState::new(default_commits));

        let default_job_results = vec![];

        finished_job_dispatch.set(FinishedJobState::new(default_job_results));
    } else {
        use crate::queue::QueueState;
        if finished_job_dispatch.get().jobs.is_empty() {
            spawn_local(async {
                let jobs = hardcoded_profiling_jobs();
                let queue_dispatch = Dispatch::<QueueState>::new();
                for job in jobs {
                    let mut queue = QueueState::clone(&queue_dispatch.get());
                    queue.queue.push_back(job.clone());
                    queue_dispatch.set(QueueState::clone(&queue));
                    use gloo_net::http::{Method, Request};
                    let resp = Request::get("/api/job")
                        .method(Method::POST)
                        .json(&job)
                        .unwrap() // This should be impossible to fail.
                        .send()
                        .await
                        .expect("Server didn't respond. Is it running?");
                    log!("Sent request got: ", format!("{resp:?}"));
                }
            });
        }
    }
    yew::Renderer::<App>::new().render();
}
