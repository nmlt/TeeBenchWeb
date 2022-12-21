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

#[function_component]
fn UploadCommit() -> Html {
    let onchange = {
        let (_store, dispatch) = use_store::<CommitState>();
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
                    store.commits.push(commit.clone());
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
    html! {
        <>
        <input type="file" {onchange} />
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
