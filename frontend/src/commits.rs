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

use common::data_types::{Algorithm, Commit, Operator, VariantNames};

use yew_router::components::Link;

use crate::Route;

#[derive(Debug, Clone, PartialEq, Default, Store)]
pub struct CommitState {
    pub commits: Vec<Commit>,
    // TODO Would it be a good idea to put another field in here that encodes an error to communicate with the server? Depending on its value the commit list could display a field to reload the list.
}

impl CommitState {
    pub fn new(commits: Vec<Commit>) -> Self {
        CommitState { commits }
    }
}

/// Holds the data from the upload form.
// It's not useful for this to be an Option, because as soon as there is eg. an uploaded file, that Option becomes Some. But that doesn't mean that the title field has been filled in, so the commit might still have bogus in the title field, even though the code field is already ok.
#[derive(Debug, Clone, PartialEq, Default, Store)]
pub struct UploadCommitState(Commit);

/// Holds the selected baseline from the upload form.
#[derive(Debug, Clone, PartialEq, Default, Store)]
pub struct BaselineState(Algorithm);

#[function_component]
fn UploadCommit() -> Html {
    let onchange = {
        let dispatch = Dispatch::<UploadCommitState>::new();
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
                    store.0.code = code;
                }
            })
        })
    };
    let onchange_title = {
        let dispatch = Dispatch::<UploadCommitState>::new();
        dispatch.reduce_mut_callback_with(|store, e: Event| {
            let input_elem = e.target_unchecked_into::<HtmlInputElement>();
            store.0.title = input_elem.value();
        })
    };
    let onclick = {
        let upload_commit_state = use_store_value::<UploadCommitState>();
        let dispatch = Dispatch::<CommitState>::new();
        dispatch.reduce_mut_future_callback_with(move |commit_state, e: MouseEvent| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            let form: HtmlFormElement = input.form().unwrap();
            form.reset();
            let upload_commit_state = upload_commit_state.clone();
            Box::pin(async move {
                let mut new_commit = upload_commit_state.0.clone();
                new_commit.datetime = OffsetDateTime::now_utc();
                commit_state.commits.push(new_commit.clone());
                let _resp = Request::get("/api/commit")
                    .method(Method::POST)
                    .json(&new_commit)
                    .unwrap()
                    .send()
                    .await
                    .unwrap();
                //log!("Sent commit to server, got response: ", format!("{_resp:?}"));
            })
        })
    };
    let operators = Operator::VARIANTS;
    let operators = SelectDataOption::options_vec(&operators);
    let operators_onchange = {
        let (_, dispatch) = use_store::<UploadCommitState>();
        dispatch.reduce_mut_callback_with(|store, e: Event| {
            let select_elem = e.target_unchecked_into::<HtmlSelectElement>();
            let value = select_elem.value();
            store.0.operator = Operator::from_str(&value).unwrap();
        })
    };
    let upload_commit_store = use_store_value::<UploadCommitState>();
    let algs = Algorithm::VARIANTS;
    let algs = SelectDataOption::options_vec(&algs);
    let algs_onchange = {
        let dispatch = Dispatch::<BaselineState>::new();
        dispatch.reduce_mut_callback_with(|store, e: Event| {
            let select_elem = e.target_unchecked_into::<HtmlSelectElement>();
            let value = select_elem.value();
            store.0 = Algorithm::from_str(&value).unwrap();
        })
    };
    let baseline_store = use_store_value::<BaselineState>();
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
                        <InputSelect options={operators} onchange={operators_onchange} label={"Operator"} multiple={false} selected={vec![upload_commit_store.0.operator.to_string()]} disabled={false} />
                    </div>
                    <div class="col-md">
                        <InputSelect options={algs} onchange={algs_onchange} label={"Baseline"} multiple={false} selected={vec![baseline_store.0.to_string()]} disabled={false} />
                    </div>
                    <div class="col-auto">
                        <input class="btn btn-primary" type="button" {onclick} value={"Upload"} />
                    </div>
                </div>
            </div>
        </form>
    }
}

#[derive(Properties, PartialEq)]
struct CommitsListProps {
    commits: Vec<Commit>,
}

#[function_component]
fn CommitsList(CommitsListProps { commits }: &CommitsListProps) -> Html {
    let (_content_store, content_dispatch) = use_store::<ModalContent>();

    let list_items_html: Html = commits.iter().rev().map(|commit| {
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

    html! {
        <li class="list-group-item">
            <b>{format!("{}", commit.title.clone())}</b>

            <div class="container d-flex flex-row justify-content-start">
                <div class="p-2"><div class="btn btn-light">{commit.operator}</div></div>
                <div class="p-2">
                    <button type="button" class="btn btn-secondary" {onclick} data-bs-toggle="modal" data-bs-target="#mainModal">{"Code"}</button>
                </div>
                <div class="p-2">
                    <Link<Route> classes={classes!("btn", "btn-info")} to={Route::PerfReport { commit: commit.title }}>
                        <span>{"Report"}</span>
                    </Link<Route>>
                </div>
            </div>
        </li>
    }}).collect();
    html! {
        <ul class="list-group">
            {list_items_html}
        </ul>
    }
}

#[function_component]
pub fn Commits() -> Html {
    let (commit_state, commit_dispatch) = use_store::<CommitState>();
    use_effect_with_deps(
        move |_| {
            let commit_dispatch = commit_dispatch.clone();
            spawn_local(async move {
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
    let commits = (*commit_state).commits.clone();
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
                            <CommitsList commits={commits} />
                        </div>
                    </main>
                </div>
            </div>
            <Modal />
        </div>
    }
}
