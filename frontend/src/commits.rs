use gloo_console::log;
use gloo_file::{futures::read_as_text, File};
use gloo_net::http::{Method, Request};
use js_sys;
use time::OffsetDateTime;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlFormElement, HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;
use yewdux::prelude::*;

use std::str::FromStr;

use crate::chartjs::hljs_highlight;
use crate::components::select::{InputSelect, SelectDataOption};
use crate::modal::Modal;
use crate::modal::ModalContent;
use crate::navigation::Navigation;

use common::data_types::{Algorithm, Commit, Job, JobConfig, JobStatus, Operator, VariantNames};

use yew_router::components::Link;

use crate::Route;

// TODO This is probably a bad idea. Just put the counter in the UploadCommitFormState. But then I have again the problem of Default.
static mut COMMIT_ID_COUNTER: usize = 0;

#[derive(Debug, Clone, PartialEq)]
/// 0: the commit
/// 1: compilation status:
pub struct CommitStatus {
    pub commit: Commit,
    /// - None: not yet compiled
    /// - Some(Ok((false, ""))): compiling
    /// - Some(Ok((true, "warnings"))): compiled without errors,
    /// - Some(Err("errors")): compiled with errors
    pub compile_status: Option<Result<(bool, String), String>>,
    pub reports_exps_running: bool,
}

impl CommitStatus {
    pub fn new(commit: Commit) -> Self {
        Self {
            commit,
            compile_status: None,
            reports_exps_running: false,
        }
    }
}

// TODO Would it be a good idea to put another field in here that encodes an error to communicate with the server? Depending on its value the commit list could display a field to reload the list.
#[derive(Debug, Clone, PartialEq, Default, Store)]
pub struct CommitState(pub Vec<CommitStatus>);

impl CommitState {
    pub fn new(commits: Vec<Commit>) -> Self {
        Self(commits.into_iter().map(|c| CommitStatus::new(c)).collect())
    }
}

/// Holds the data from the upload form.
// It's not useful for this to be an Option, because as soon as there is eg. an uploaded file, that Option becomes Some. But that doesn't mean that the title field has been filled in, so the commit might still have bogus in the title field, even though the code field is already ok.
/// TODO Instead this struct could represent the form, with each available field being an option.
#[derive(Debug, Clone, PartialEq, Default, Store)]
pub struct UploadCommitFormState {
    pub title: Option<String>,
    pub operator: Option<Operator>,
    pub code: Option<String>,
    pub baseline: Option<Algorithm>,
}

impl UploadCommitFormState {
    // TODO Can this be converted to some From<Commit> implementation and use the automatic into?
    /// Only call after you verified that the form has been filled in correctly. Otherwise this panics
    pub fn to_commit_and_baseline(&self) -> (Commit, Algorithm) {
        let c;
        unsafe {
            c = Commit {
                title: self.title.clone().unwrap(),
                operator: self.operator.clone().unwrap(),
                datetime: OffsetDateTime::now_utc(),
                code: self.code.clone().unwrap(),
                reports: vec![],
                findings: vec![],
                id: COMMIT_ID_COUNTER,
            };
            COMMIT_ID_COUNTER += 1;
        }
        (c, self.baseline.clone().unwrap())
    }
    pub fn verify(&self) -> bool {
        self.title.is_some()
            && self.operator.is_some()
            && self.code.is_some()
            && self.baseline.is_some()
    }
    pub fn reset(&mut self) {
        self.title = None;
        self.operator = None;
        self.code = None;
        self.baseline = None;
    }
}

#[function_component]
fn UploadCommit() -> Html {
    let onchange = {
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
    let algs = SelectDataOption::options_vec(&algs);
    let algs_onchange = {
        let dispatch = Dispatch::<UploadCommitFormState>::new();
        dispatch.reduce_mut_callback_with(|store, e: Event| {
            let select_elem = e.target_unchecked_into::<HtmlSelectElement>();
            let value = select_elem.value();
            store.baseline = Some(Algorithm::from_str(&value).unwrap());
        })
    };
    let onclick = {
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
                let (new_commit, baseline) = upload_commit_state.to_commit_and_baseline();
                let id = new_commit.id;
                commit_state.0.push(CommitStatus::new(new_commit.clone()));
                let _resp = Request::get("/api/commit")
                    .method(Method::POST)
                    .json(&new_commit)
                    .unwrap()
                    .send()
                    .await
                    .unwrap();
                //log!("Sent commit to server, got response: ", format!("{_resp:?}"));
                upload_commit_dispatch.reduce_mut(|s| s.reset());

                // Send compile job
                let compile_job = Job {
                    config: JobConfig::Compile(id),
                    submitted: OffsetDateTime::now_utc(),
                    status: JobStatus::default(),
                };
                let _resp = Request::get("/api/job")
                    .method(Method::POST)
                    .json(&compile_job)
                    .unwrap()
                    .send()
                    .await
                    .unwrap();
                // TODO Instead of unwrapping show a possible error while sending.
                commit_state.0.last_mut().unwrap().compile_status =
                    Some(Ok((false, "".to_string())));
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
    html! {
        <form class="row g-3">
            <div class="col-md">
                <div class="row g-3">
                    <div class="col-md">
                        <div>
                            <label for="uploadFormFile" class="form-label">{"Source code file"}</label>
                            <input id="uploadFormFile" class="form-control" type="file" {onchange} />
                        </div>
                    </div>
                    <div class="col-md">
                        <div>
                            <label for="titleFormInput" class="form-label">{"Commit message"}</label>
                            <input id="titleFormInput" class="form-control" type="text" onchange={onchange_title} />
                        </div>
                    </div>
                    <div class="col-md">
                        <InputSelect options={operators} onchange={operators_onchange} label={"Operator"} multiple={false} selected={selected_operator} disabled={false} />
                    </div>
                    <div class="col-md">
                        <InputSelect options={algs} onchange={algs_onchange} label={"Baseline"} multiple={false} selected={selected_baseline} disabled={false} />
                    </div>
                    <div class="col-auto">
                        <input class="btn btn-primary" type="button" {onclick} disabled={upload_disabled} value={"Upload"} />
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

    let list_items_html: Html = commit_store.0.iter().rev().map(|CommitStatus {commit, compile_status, reports_exps_running}| {
        let commit = commit.clone();

        let onclick = {
            let content_dispatch = content_dispatch.clone();
            let commit = commit.clone();
            content_dispatch.set_callback(move |_| {
                let commit = commit.clone();
                let html = hljs_highlight(commit.code);
                let parsed = Html::from_html_unchecked(AttrValue::from(format!("<code class=\"hljs language-cpp\">{html}</code>")));
                ModalContent {
                    content: html! {
                        <div class="modal-content">
                            <div class="modal-header">
                                <h5 class="modal-title">{commit.title}</h5>
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
                    }
                }
            })
        };
        let compile_status_view = match compile_status {
            // - None: not yet compiled
            None => html! {"waiting to start compilation..."},
            // - Some(Ok((false, ""))): compiling
            Some(Ok((false, _))) => html! {"compiling..."},
            // - Some(Ok((true, "warnings"))): compiled without errors,
            Some(Ok((true, warnings))) => {
                if warnings.is_empty() {
                    html! {"Successfully compiled."}
                } else {
                    html! {
                        <>
                        {"There were some warnings."}
                        {warnings}
                        </>
                    }
                }
            }
            // - Some(Err("errors")): compiled with errors
            Some(Err(e)) => {
                html! {
                    <>
                    {"Failed to compile"}
                    {e}
                    </>
                }
            }
        };
        let report_button = if *reports_exps_running {
            html! {
                <Link<Route> classes={classes!("btn", "btn-info")} to={Route::PerfReport { commit: commit.title.clone() }}>
                    {"Report"}
                    <span class="spinner-border spinner-border-sm" role="status" aria-hidden="true"></span>
                </Link<Route>>
            }
        } else {
            let onclick = {
                let commit_dispatch = commit_dispatch.clone();
                let id = commit.id;
                // Send perf_report job
                commit_dispatch.reduce_mut_callback(move |s| {
                    for c in s.0.iter_mut() {
                        if c.commit.id == id {
                            c.reports_exps_running = true;
                            break;
                        }
                    }
                })
            };
            html! { <button class="btn btn-info" {onclick}>{"Report"}</button> }
        };
        html! {
            <li class="list-group-item">
                <b>{format!("{}", commit.title.clone())}</b>

                <div class="container d-flex flex-row justify-content-start">
                    <div class="p-2"><div class="btn btn-light">{commit.operator}</div></div>
                    <div class="p-2">
                        <button class="btn btn-secondary" {onclick} data-bs-toggle="modal" data-bs-target="#mainModal">{"Code"}</button>
                    </div>
                    <div class="p-2">
                        {compile_status_view}
                    </div>
                    <div class="p-2">
                        {report_button}
                    </div>
                </div>
            </li>
        }
    }).collect();
    html! {
        <ul class="list-group">
            {list_items_html}
        </ul>
    }
}

#[function_component]
pub fn Commits() -> Html {
    let (_commit_state, commit_dispatch) = use_store::<CommitState>();
    use_effect_with_deps(
        move |_| {
            let commit_dispatch = commit_dispatch.clone();
            spawn_local(async move {
                // TODO Get the largest commit id and save it in COMMIT_ID_COUNTER (unsafe).
                let commit_dispatch = commit_dispatch.clone();
                let resp: Result<Vec<Commit>, _> = Request::get("/api/commit")
                    .method(Method::GET)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await;
                //log!(format!("GET /api/commits: Response: {:?}", resp));
                match resp {
                    Ok(json) => {
                        //log!(format!("got commits: {json:?}"));
                        commit_dispatch.set(CommitState::new(json));
                    }
                    Err(e) => log!("Error getting commit json: ", e.to_string()),
                }
            });
        },
        (),
    );
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
