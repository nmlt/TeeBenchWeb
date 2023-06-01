use gloo_console::log;
use gloo_file::{futures::read_as_text, File};
use gloo_net::http::{Method, Request};
use js_sys;
use time::OffsetDateTime;
use web_sys::{HtmlFormElement, HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;
use yewdux::prelude::*;

use std::str::FromStr;

use crate::components::select::{InputSelect, SelectDataOption};
use crate::components::tag::Tag;
use crate::js_bindings::{diff2html_html, hljs_highlight};
use crate::modal::Modal;
use crate::modal::ModalContent;
use crate::navigation::Navigation;
use crate::queue::QueueState;

use common::commit::{
    CommitState, CompilationStatus, Operator, PerfReportStatus, UploadCommitFormState,
};
use common::data_types::{Algorithm, Job, JobConfig, PerfReportConfig, VariantNames};

use yew_router::components::Link;

use crate::Route;

#[function_component]
fn UploadCommit() -> Html {
    let commit_store = use_store_value::<CommitState>();
    let commit_store_moved = commit_store.clone();
    // TODO: I think this is unnecessary
    // use_effect_with_deps(
    //     move |_| {
    //         let largest = commit_store_moved.get_latest().map(|c| c.id).unwrap_or(0);
    //         unsafe {
    //             // COMMIT_ID_COUNTER = largest;
    //         }
    //     },
    //     commit_store.clone(),
    // );
    let onchange_file = {
        let dispatch = Dispatch::<UploadCommitFormState>::new();
        dispatch.reduce_mut_future_callback_with(|store, e: Event| {
            Box::pin(async move {
                //log!("UploadCommit: onchange triggered!");
                let input = e.target_unchecked_into::<HtmlInputElement>();
                if let Some(file_list) = input.files() {
                    let file: File = js_sys::try_iter(&file_list)
                        .unwrap()
                        .unwrap()
                        .next()
                        .map(|v| web_sys::File::from(v.unwrap()))
                        .map(File::from)
                        .unwrap();
                    let code = read_as_text(&file).await.unwrap();
                    store.code = Some(code);
                }
            })
        })
    };
    let onchange_title = {
        let dispatch = Dispatch::<UploadCommitFormState>::new();
        dispatch.reduce_mut_callback_with(|store, e: Event| {
            let input_elem = e.target_unchecked_into::<HtmlInputElement>();
            store.title = Some(input_elem.value());
        })
    };
    let onchange_version = {
        let dispatch = Dispatch::<UploadCommitFormState>::new();
        dispatch.reduce_mut_callback_with(|store, e: Event| {
            let input_elem = e.target_unchecked_into::<HtmlInputElement>();
            store.version = Some(input_elem.value());
        })
    };
    let operators = Operator::VARIANTS;
    let operators = SelectDataOption::options_vec(&operators);
    // TODO Add a "Select an Operator" option that maps to None in the store.
    let operators_onchange = {
        let (_, dispatch) = use_store::<UploadCommitFormState>();
        dispatch.reduce_mut_callback_with(|store, e: Event| {
            let select_elem = e.target_unchecked_into::<HtmlSelectElement>();
            let value = select_elem.value();
            store.operator = Some(Operator::from_str(&value).unwrap());
        })
    };
    let algs = Algorithm::VARIANTS;
    let algs = {
        let mut algs = SelectDataOption::options_vec(algs);
        let found = algs.iter_mut().find(|o| o.value == "Latest Operator");
        if found.is_some() {
            let o: &mut SelectDataOption = found.unwrap(); // Putting &mut in front of the variable does not work. Type just to understand.
            if let Some(c) = commit_store.get_latest() {
                o.label = format!("Latest Operator ({})", c.title).to_string();
                // We could put the id of the commit in the value field to offer not just the latest commit, but all.
            } else {
                o.enabled = false;
            }
        };
        algs
    };
    let algs_onchange = {
        let dispatch = Dispatch::<UploadCommitFormState>::new();
        let commit_store = commit_store.clone();
        dispatch.reduce_mut_callback_with(move |store, e: Event| {
            let select_elem = e.target_unchecked_into::<HtmlSelectElement>();
            let value = select_elem.value();
            let value = Algorithm::from_str(&value).unwrap();
            store.baseline = match value {
                Algorithm::Commit(_) => {
                    let id = match commit_store.get_latest() {
                        Some(c) => c.id,
                        None => {
                            log!("Error: No latest operator yet. Upload code in the Operator tab!");
                            panic!("This should not have been possible to happen :(");
                        }
                    };
                    Some(Algorithm::Commit(id))
                }
                alg_variant => Some(alg_variant),
            };
        })
    };
    let onclick_submit = {
        let (upload_commit_state, upload_commit_dispatch) = use_store::<UploadCommitFormState>();
        let dispatch = Dispatch::<CommitState>::new();
        dispatch.reduce_mut_future_callback_with(move |commit_state, e: MouseEvent| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            let form: HtmlFormElement = input.form().unwrap();
            form.reset();
            let upload_commit_state = upload_commit_state.clone();
            let upload_commit_dispatch = upload_commit_dispatch.clone();
            Box::pin(async move {
                // Verified that the UploadCommitFormState has no fields with None by disabling this callback's button until the condition is met.
                let new_commit = upload_commit_state.to_commit();
                let id = new_commit.id;
                commit_state.0.push(new_commit.clone());
                let _resp = Request::get("/api/commit")
                    .method(Method::POST)
                    .json(&new_commit)
                    .unwrap()
                    .send()
                    .await
                    .expect("Server didn't respond. Is it running?");
                //log!("Sent commit to server, got response: ", format!("{_resp:?}"));
                upload_commit_dispatch.reduce_mut(|s| s.reset());

                // Send compile job
                let compile_job = Job::new(JobConfig::Compile(id), OffsetDateTime::now_utc());
                let _resp = Request::get("/api/job")
                    .method(Method::POST)
                    .json(&compile_job)
                    .unwrap()
                    .send()
                    .await
                    .expect("Server didn't respond. Is it running?");
                // TODO Instead of unwrapping show a possible error while sending.
                // TODO If there is already a job, the compilationstatus should be not compiled.
                commit_state.0.last_mut().unwrap().compilation = CompilationStatus::Compiling;
            })
        })
    };
    let upload_commit_store = use_store_value::<UploadCommitFormState>();
    // Those next two statements do essentially the same.
    let selected_operator = upload_commit_store
        .operator
        .as_ref()
        .map(|s| vec![s.to_string()])
        .unwrap_or(vec![]);
    let selected_baseline = if let Some(selected_baseline) = &upload_commit_store.baseline {
        vec![selected_baseline.to_string()]
    } else {
        vec![]
    };
    let upload_disabled = !upload_commit_store.verify();
    let entire_form_disabled = if cfg!(feature = "static") {
        true
    } else {
        false
    };
    html! {
        <form class="row g-3">
            <div class="col-md">
                <div class="row g-3">
                    <div class="col-md">
                        <div>
                            <label for="uploadFormFile" class="form-label">{"Source code"}</label>
                            <input id="uploadFormFile" class="form-control" type="file" onchange={onchange_file} disabled={entire_form_disabled} />
                        </div>
                    </div>
                    <div class="col-md">
                        <div>
                            <label for="titleFormInput" class="form-label">{"Title"}</label>
                            <input id="titleFormInput" class="form-control" type="text" onchange={onchange_title} disabled={entire_form_disabled} />
                        </div>
                    </div>
                    <div class="col-md">
                        <div>
                            <label for="versionFormInput" class="form-label">{"Version"}</label>
                            <input id="versionFormInput" class="form-control" type="text" onchange={onchange_version} disabled={entire_form_disabled} />
                        </div>
                    </div>
                    <div class="col-md">
                        <InputSelect options={operators} onchange={operators_onchange} label={"Operator"} multiple={false} selected={selected_operator} disabled={entire_form_disabled} />
                    </div>
                    <div class="col-md">
                        <InputSelect options={algs} onchange={algs_onchange} label={"Baseline"} multiple={false} selected={selected_baseline} disabled={entire_form_disabled} />
                    </div>
                    <div class="col-auto">
                        <input class="btn btn-primary" type="button" onclick={onclick_submit} disabled={upload_disabled} value={"Upload"} />
                    </div>
                </div>
            </div>
        </form>
    }
}

#[function_component]
fn CommitsList() -> Html {
    let (_content_store, content_dispatch) = use_store::<ModalContent>();
    let (commit_store, commit_dispatch) = use_store::<CommitState>();
    let queue_dispatch = Dispatch::<QueueState>::new();

    let diffs = commit_store.get_diffs();

    let current_date = commit_store.get_latest().map(|c| c.get_date());
    let list_items_html = commit_store.0.iter().rev().zip(diffs.iter().rev()).map(|(commit, diff)| {
        let commit = commit.clone();

        let onclick_code = {
            let content_dispatch = content_dispatch.clone();
            let commit = commit.clone();
            content_dispatch.set_callback(move |_| {
                let commit = commit.clone();
                let html = hljs_highlight(commit.code.clone());
                let parsed = Html::from_html_unchecked(AttrValue::from(format!("<code class=\"hljs language-cpp\">{html}</code>")));
                ModalContent::new(html! {
                    <div class="modal-content">
                        <div class="modal-header">
                            <h5 class="modal-title">{commit.get_title()}</h5>
                            <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                        </div>
                        <div class="modal-body">
                            <pre>
                                {parsed}
                            </pre>
                        </div>
                        <div class="modal-footer">
                            <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{"Close"}</button>
                        </div>
                    </div>
                })
            })
        };
        let commit_title = commit.get_title();
        let compile_status_view = match commit.compilation {
            CompilationStatus::Uncompiled => html! {"waiting to start compilation..."},
            CompilationStatus::Compiling => html! {"compiling..."},
            CompilationStatus::Successful(ref warnings) => {
                if warnings.is_empty() {
                    html! {"Successfully compiled."}
                } else {
                    let compiler_output_onclick = {
                        let content_dispatch = content_dispatch.clone();
                        let warnings = warnings.clone();
                        content_dispatch.set_callback(move |_| {
                            let warnings = warnings.clone();
                            ModalContent::with_modal_skeleton(html! {
                                <pre>
                                    {warnings}
                                </pre>
                            }, html! {{format!("Compiler output for {commit_title}")}})
                        })
                    };
                    html! {
                        <button class="btn btn-success" onclick={compiler_output_onclick} data-bs-toggle="modal" data-bs-target="#mainModal">{"Show Compiler Output"}</button>
                    }
                }
            }
            CompilationStatus::Failed(ref e) => {
                let compiler_output_onclick = {
                    let content_dispatch = content_dispatch.clone();
                    let e = e.clone();
                    content_dispatch.set_callback(move |_| {
                        let e = e.clone();
                        ModalContent::with_modal_skeleton(html! {
                            <pre>
                                {e}
                            </pre>
                        }, html! {{format!("Compiler output for {commit_title}")}})
                    })
                };
                html! {
                    <button class="btn btn-danger" onclick={compiler_output_onclick} data-bs-toggle="modal" data-bs-target="#mainModal">{"Show Compiler Output"}</button>
                }
            }
        };
        let report_button = match commit.perf_report_running {
            PerfReportStatus::None => {
                if let CompilationStatus::Successful(_) = commit.compilation {
                    let onclick = {
                        let commit_dispatch = commit_dispatch.clone();
                        let queue_dispatch = queue_dispatch.clone();
                        let id = commit.id;
                        let baseline = commit.baseline;
                        commit_dispatch.reduce_mut_future_callback(move |store| {
                            let queue_dispatch = queue_dispatch.clone();
                            Box::pin(async move {
                                // TODO Passing fit in here and disregarding _exceed?
                                let (fit, _exceed) = PerfReportConfig::for_throughput(id, baseline);
                                let perf_report_job = Job::new(JobConfig::PerfReport(fit), OffsetDateTime::now_utc());
                                let _resp = Request::get("/api/job")
                                    .method(Method::POST)
                                    .json(&perf_report_job)
                                    .unwrap()
                                    .send()
                                    .await
                                    .expect("Server didn't respond. Is it running?");
                                for c in store.0.iter_mut() {
                                    if c.id == id {
                                        c.perf_report_running = PerfReportStatus::Running(perf_report_job.id);
                                        break;
                                    }
                                }
                                queue_dispatch.reduce_mut(|queue| {
                                    queue.queue.push_back(perf_report_job);
                                });
                            })
                        })
                    };
                    html! { <button class="btn btn-info" {onclick}>{"Generate Report"}</button> }
                } else {
                    html! {
                        <button class="btn btn-info" disabled={true}>{"Report"}</button>
                    }
                }
            }
            PerfReportStatus::Running(id) => html! {
                <>
                <Link<Route> classes={classes!("btn", "btn-info")} to={Route::PerfReport { name: commit.get_title() }}>
                    {"Report"}
                    <span class="spinner-border spinner-border-sm" role="status" aria-hidden="true"></span>
                </Link<Route>>
                <RemovePerfReportButton id={id} />
                </>
            },
            PerfReportStatus::Successful => html! {
                <Link<Route> classes={classes!("btn", "btn-info")} to={Route::PerfReport { name: commit.get_title() }}>
                    {"View Report"}
                </Link<Route>>
            },
            PerfReportStatus::Failed => html! {
                <button class="btn btn-info" disabled={true}>{"Report"}</button>
            }
        };
        let baseline = {
            match commit.baseline {
                Algorithm::Commit(id) => commit_store.get_title(&id).unwrap(),
                alg => alg.to_string(),
            }
        };
        let diff_button = {
            if let Some(diff) = diff {
                let diff = diff.clone();
                let diff_onclick = content_dispatch.set_callback(move |_| {
                    let diff = diff.clone();
                    // This is only so that the highlighter works, the file name and dates are removed in the `MyChart.js` file.
                    let diff = format!("--- from_file.cpp 2002-02-21 23:30:39\n+++ to_file.cpp 2022-02-21 23:30:39\n{diff}");
                    let diff = diff2html_html(diff);
                    let parsed = Html::from_html_unchecked(AttrValue::from(diff));
                    // TODO Add a header to the modal with operator titles.
                    ModalContent::with_modal_skeleton(html! {
                        {parsed}
                    }, html!{ "Diff" })
                });
                html! {
                    <button class="btn btn-info" onclick={diff_onclick} data-bs-toggle="modal" data-bs-target="#mainModal">{"View Diff"}</button> 
                }
           } else {
                html! {}
            }
        };
        (commit.get_date(), html! {
            <li class="list-group-item">
                <b>{commit.get_title()}</b>
                {" "}
                <span class="fs-6 text-muted">{commit.get_time_of_day()}</span>

                <div class="container d-flex flex-row justify-content-start">
                    <div class="p-2"><div class="btn btn-light">{commit.operator}</div></div>
                    <div class="p-2">
                        <button class="btn btn-secondary" onclick={onclick_code} data-bs-toggle="modal" data-bs-target="#mainModal">{"Code"}</button>
                    </div>
                    <div class="p-2">
                        {compile_status_view}
                    </div>
                    <div class="p-2">
                        {report_button}
                    </div>
                    <div class="p-2">
                        {diff_button}
                    </div>
                    <div class="p-2">
                        {"Baseline: "}
                        <Tag text={baseline} />
                    </div>
                </div>
            </li>
        })
    }).fold((current_date, vec![]), |acc, (d, h)| {
        // Partition the list items according to the date they were uploaded. Relies on them being sorted by this date.
        let curr_date = acc.0.unwrap();
        // The String in v is the date.
        let mut v: Vec<(String, Vec<Html>)> = acc.1;
        if curr_date == d {
            if let Some(ref mut bucket) = v.last_mut() {
                bucket.1.push(h);
            } else {
                // Only happens for the first commit.
                v.push((d, vec![h]));
            }
        } else {
            v.push((d, vec![h]));
        }
        (Some(curr_date), v)
    }).1.into_iter().map(|v| {
        html! {
            <>
            <b class="fs-5">{v.0}</b>
            <ul class="list-group">
                {for v.1}
            </ul>
            </>
        }
    });
    html! {
        {for list_items_html}
    }
}

#[function_component]
pub fn Commits() -> Html {
    html! {
        <div class="container-fluid">
            <div class="row vh-100">
                <div class="col-12 col-sm-3 col-xl-2 px-sm-2 px-0 bg-dark d-flex sticky-top">
                    <Navigation active_nav_item={"Commits"} />
                </div>
                <div class="col d-flex flex-column h-sm-100">
                    <main class="row">
                        <div class="col pt-4 col-lg-8">
                            <h2>{"Operators"}</h2>
                            <UploadCommit />
                            <CommitsList />
                        </div>
                    </main>
                </div>
            </div>
            <Modal />
        </div>
    }
}

use crate::components::websocket::WebsocketState;
use common::data_types::{ClientMessage, JobIdType};
#[derive(PartialEq, Properties)]
pub struct RemovePerfReportButtonProps {
    pub id: JobIdType,
}

#[function_component]
pub fn RemovePerfReportButton(
    RemovePerfReportButtonProps { id }: &RemovePerfReportButtonProps,
) -> Html {
    let (commit_store, commit_dispatch) = use_store::<CommitState>();
    let empty = commit_store.0.is_empty();
    let onclick = {
        let websocket_store = use_store_value::<WebsocketState>();
        let id = id.clone();
        commit_dispatch.reduce_mut_callback(move |s| {
            websocket_store.send(ClientMessage::RemoveJob(id));
            if let Some(ref mut found) = s.0
                .iter_mut()
                .find(|c| matches!(c.perf_report_running, PerfReportStatus::Running(incoming_id) if incoming_id == id))
            {
                found.perf_report_running = PerfReportStatus::None;
            }
            // TODO Also remove from queue as it's in there now too, isn't it?
        })
    };
    html! {
        <button class="btn btn-danger" disabled={empty} {onclick}>
            <i class="bi-stop-fill"></i>
        </button>
    }
}
