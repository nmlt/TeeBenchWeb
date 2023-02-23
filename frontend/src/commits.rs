use gloo_console::log;
use gloo_file::{futures::read_as_text, File};
use gloo_net::http::{Method, Request};
use js_sys;
use std::{rc::Rc};
use time::OffsetDateTime;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::chartjs::hljs_highlight;
use crate::modal::Modal;
use crate::modal::ModalContent;
use crate::navigation::Navigation;

use common::data_types::Commit;

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

#[derive(Debug, Clone, PartialEq, Default, Store)]
pub struct UploadCommitState(Option<Commit>);

#[function_component]
fn UploadCommit() -> Html {
    let onchange = {
        let (_store, dispatch) = use_store::<UploadCommitState>();
        dispatch.reduce_mut_future_callback_with(|store, e: Event| {
            Box::pin(async move {
                log!("UploadCommit: onchange triggered!");
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
                    let commit = Commit::new(
                        "placeholder".to_owned(),
                        "JOIN".to_owned(),
                        OffsetDateTime::now_utc(),
                        code,
                        vec![],
                    );
                    store.0 = Some(commit.clone());
                    let resp = Request::get("/api/commit")
                        .method(Method::POST)
                        .json(&commit)
                        .unwrap()
                        .send()
                        .await
                        .unwrap();
                    log!("Sent commit to server, got response: ", format!("{resp:?}"));
                }
            })
        })
    };
    let onchange_title = {
        let (_state, dispatch) = use_store::<UploadCommitState>();
        dispatch.reduce_callback_with(|s, e: Event| {
            let input_elem = e.target_unchecked_into::<HtmlInputElement>();
            let mut c = s.0.clone().unwrap();
            c.title = input_elem.value();
            Rc::new(UploadCommitState(Some(c)))
        })
    };
    let onclick = {
        let (upload_commit_state, _dispatch) = use_store::<UploadCommitState>();
        let (_store, dispatch) = use_store::<CommitState>();
        dispatch.reduce_mut_future_callback(move |commit_state| {
            let upload_commit_state = upload_commit_state.clone();
            Box::pin(async move {
                let upload_commit_state = upload_commit_state.clone();
                if let Some(new_commit) = upload_commit_state.0.clone() {
                    commit_state.commits.push(new_commit);
                    // TODO Remove the new_commit from the UploadCommitState. Not possible here, because it's a Rc without a RefCell. Not possible with a callback, because the callback needs the state as argument. And I can't call hooks in here. So maybe I have to use a channel? Or switch this around and make this callback run on UploadCommitState instead of CommitState?
                }
            })
        })
    };
    html! {
        <>
        <input type="file" {onchange} />
        <input type="text" onchange={onchange_title} />
        <select class="custom-select">
          <option selected=true>{"Select Operator..."}</option>
          <option value="1">{"JOIN"}</option>
          <option value="2">{"GROUP BY"}</option>
          <option value="3">{"PROJECTION"}</option>
          <option value="4">{"ORDER BY"}</option>
        </select>
        <select class="custom-select">
          <option selected=true>{"Select Baseline..."}</option>
          <option value="1">{"MWAY"}</option>
          <option value="2">{"PHT"}</option>
          <option value="3">{"CHT"}</option>
          <option value="4">{"RHT"}</option>
        </select>
        <button type="button" {onclick}>{"Upload"}</button>
        </>
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
                        log!(format!("got commits: {json:?}"));
                        commit_dispatch.set(CommitState::new(json));
                    },
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
