use yew::prelude::*;
use yewdux::prelude::*;
use gloo_console::log;
use gloo_net::http::{Method, Request};
use wasm_bindgen_futures::spawn_local;
use time::OffsetDateTime;
use web_sys::HtmlInputElement;
use gloo_file::{File, futures::read_as_text};
use js_sys;
use std::{pin::Pin, rc::Rc};

use crate::navigation::Navigation;

use common::data_types::Commit;

#[derive(Debug, Clone, PartialEq, Default, Store)]
pub struct CommitState {
    commits: Vec<Commit>,
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
                    let commit = Commit::new("placeholder".to_owned(), OffsetDateTime::now_utc(), code, None);
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
    let list_items_html: Html = commits.iter().map(|commit| html! {
        <li class="list-group-item">{format!("Commit: {}\n {:?}", commit.title, commit.report)}</li>
    }).collect();
    html! {
        <ul class="list-group">
            {list_items_html}
        </ul>
    }
}

impl CommitState {
    fn new(commits: Vec<Commit>) -> Self {
        CommitState {
            commits
        }
    }
}

#[function_component]
pub fn Commits() -> Html {
    let (commit_state, _dispatch) = use_store::<CommitState>();
    {   
        let commit_state = commit_state.clone(); 
        spawn_local(async move {
            let mut commit_state = commit_state.clone();
            let resp: Result<Vec<Commit>, _> = Request::get("/api/profiling")
                .method(Method::GET)
                .send()
                .await
                .unwrap()
                .json()
                .await;
            match resp {
                Ok(json) => Pin::new(&mut commit_state).set(Rc::new(CommitState::new(json))),
                Err(e) => log!("Error getting commit json: ", e.to_string()),
            }
            
        });
    }
    let mut commits = (*commit_state).commits.clone();
    commits.push(Commit::new("initial commit".to_owned(), OffsetDateTime::now_utc(), "auto a = 1;".to_owned(), None));
    html! {
        <div class="container-fluid">
            <div class="row vh-100">
                <div class="col-12 col-sm-3 col-xl-2 px-sm-2 px-0 bg-dark d-flex sticky-top">
                    <Navigation active_nav_item={"Commits"} />
                </div>
                <div class="col d-flex flex-column h-sm-100">
                    <main class="row">
                        <div class="col pt-4 col-lg-8">
                            <h2>{"Commits"}</h2>
                            <CommitsList commits={commits} />
                            <UploadCommit />
                        </div>
                    </main>
                </div>
            </div>
        </div>
    }
}
