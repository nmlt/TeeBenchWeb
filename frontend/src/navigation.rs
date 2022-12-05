use yew::prelude::*;
use yew_router::components::Link;

use crate::Route;

#[function_component]
pub fn Navigation() -> Html {
    html! {
        <div>
            <ul class="nav">
                <li class="nav-item">
                    <span class="nav-link">
                        <Link<Route> to={Route::Home}>
                            <i class="fs-5 bi-grid" aria-hidden="true"></i>
                            <span class="ms-1 d-none d-sm-inline">{"TeeBenchWeb"}</span>
                        </Link<Route>>
                    </span>
                </li>
                <li class="nav-item">
                    <span class="nav-link">
                        <Link<Route> to={Route::Commits}>
                            <i class="fs-5 bi-table" aria-hidden="true"></i>
                            <span class="ms-1 d-none d-sm-inline">{"Commits"}</span>
                        </Link<Route>>
                    </span>
                </li>
                <li class="nav-item">
                    <span class="nav-link">
                        <Link<Route> to={Route::Profiling}>
                            <i class="fs-5 bi-graph-up" aria-hidden="true"></i>
                            <span class="ms-1 d-none d-sm-inline">{"Profiling"}</span>
                        </Link<Route>>
                    </span>
                </li>
            </ul>
        </div>
    }
}
