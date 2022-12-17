use yew::prelude::*;
use yew_router::components::Link;

use crate::{queue::Queue, Route};

#[derive(Clone, PartialEq, Properties)]
pub struct NavigationProps {
    pub active_nav_item: String,
}

#[function_component]
pub fn Navigation(NavigationProps { active_nav_item }: &NavigationProps) -> Html {
    fn add_active_class(current: &str, active: String) -> yew::html::Classes {
        if current == &active {
            classes!("active", "nav-link", "text-primary")
        } else {
            classes!("nav-link", "text-reset")
        }
    }
    html! {
        <div>
            <nav class="d-flex flex-sm-column flex-row flex-grow-1 align-items-center align-items-sm-start px-3 pt-2 text-white">
                <ul class="nav flex-lg-column">
                    <li class="nav-item">
                        <Link<Route> classes={classes!("text-nowrap", "nav-link", "text-primary")} to={Route::Home}>
                            <i class="fs-3 bi-grid text-primary"></i>
                            <span class="fs-3 ms-1 d-none d-sm-inline text-truncate text-primary">{"TeeBenchWeb"}</span>// TODO Make this a heading.
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> classes={add_active_class("Commits", active_nav_item.clone())} to={Route::Commits}>
                            <i class="fs-5 bi-table"></i>
                            <span class="ms-1 d-none d-sm-inline">{"Commits"}</span>
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> classes={add_active_class("Profiling", active_nav_item.clone())} to={Route::Profiling}>
                            <i class="fs-5 bi-graph-up"></i>
                            <span class="ms-1 d-none d-sm-inline">{"Profiling"}</span>
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Queue />
                    </li>
                </ul>
            </nav>
        </div>
    }
}
