use common::commit::{CommitIdType, PerfReportStatus};
use common::hardcoded::hardcoded_commits;
use gloo_console::log;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

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

use common::data_types::JobResult;

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
    #[at("/performance_report/:name")]
    PerfReport { name: String },
    #[at("/performance_report/:name/:instance")]
    PerfReportDouble { name: String, instance: usize },
    #[not_found]
    #[at("/404")]
    NotFound,
}

impl Route {
    pub fn perf_report_double(name: String, instance: usize) -> Self {
        Self::PerfReportDouble { name, instance }
    }
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
            <PerfReport name={None::<String>} instance={None::<usize>} />
        },
        Route::PerfReport { name } => html! {
            <PerfReport name={Some(name)} instance={None::<usize>} />
        },
        Route::PerfReportDouble { name, instance } => html! {
            <PerfReport name={Some(name)} instance={Some(instance)} />
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
        let (hashjoin_commits, _) = hardcoded_commits();
        let mut default_commits = vec![predefined_commit()];
        let mut hashjoin_commits = hashjoin_commits
            .into_iter()
            .map(|mut c| {
                fn commit_version_to_report_json_file(
                    version: &str,
                    id: CommitIdType,
                ) -> Option<JobResult> {
                    let json = match version {
                        "1" => return None,
                        "2" => include_str!("../../cached/report_hjv2.json"),
                        "3" => include_str!("../../cached/report_hjv3.json"),
                        "4" => include_str!("../../cached/report_hjv4.json"),
                        "5" => include_str!("../../cached/report_hjv5.json"),
                        // "6" => include_str!("../../cached/report_hjv6.json"),
                        a => {
                            log!(format!("Unknown Hashjoin version: {a}"));
                            panic!();
                        }
                    };
                    let mut found = 0;
                    let mut fixed_json = String::new();
                    for line in json.lines() {
                        let line = if line.contains("\"id\": ") || line.contains("\"Commit\": ") {
                            if let Some(start) = line.find(": ") {
                                found += 1;
                                let mut fixed_line = String::new();
                                fixed_line.push_str(&line[..start + 2]);
                                fixed_line.push('"');
                                fixed_line.push_str(&id.to_string());
                                fixed_line.push('"');
                                if line.ends_with(",\n") {
                                    fixed_line.push(',');
                                }
                                fixed_line.push('\n');
                                log!(format!("{fixed_line}"));
                                fixed_line
                            } else {
                                log!("Error fixing the json ids!");
                                panic!();
                            }
                        } else {
                            line.to_owned()
                        };
                        fixed_json.push_str(&line);
                    }
                    log!(format!("Fixed json {found} times!"));
                    let serialized: JobResult = match serde_json::from_str(fixed_json.as_str()) {
                        Ok(s) => s,
                        Err(e) => {
                            log!(format!(
                                "Error serializing json for hashjoins: {e}, hashjoin version{version}"
                            ));
                            panic!();
                        }
                    };
                    Some(serialized)
                }
                let report = commit_version_to_report_json_file(&c.version, c.id);
                c.report = report;
                c.perf_report_running = match c.version.as_str() {
                    "1" => PerfReportStatus::None,
                    "2" | "3" | "4" | "5" => PerfReportStatus::Successful,
                    _ => unreachable!(),
                };

                c
            })
            .collect();
        default_commits.append(&mut hashjoin_commits);
        Dispatch::<CommitState>::new().set(CommitState::new(default_commits));

        // Path is apparently relative to `frontend/src/`
        // include_str! cannot take a variable, only literals! So no chance moving this to a variable somewhere...
        let job_results_str = include_str!("../../cached/job_results.json");
        // Yew pretty prints panics. As yew hasn't started yet, we have to pretty print them (at least a bit).
        let default_job_results: FinishedJobState = match serde_json::from_str(job_results_str) {
            Ok(parsed) => parsed,
            Err(e) => {
                log!(format!("Error parsing json: {e:#?}"));
                FinishedJobState::new(vec![])
            }
        };
        finished_job_dispatch.set(default_job_results);
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
