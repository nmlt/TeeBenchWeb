use yew::prelude::*;
use yew_router::prelude::*;

mod commits;
mod data_structures;
mod navigation;
mod profiling;

use crate::commits::Commits;
use crate::profiling::Profiling;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/commits")]
    Commits,
    #[at("/profiling")]
    Profiling,
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
            <Profiling algs={vec!["NLJ".to_string(), "CHT".to_string(), "RHO".to_string()]} exps={vec!["EPC Paging".to_string(), "throughput".to_string()]} datasets={vec!["cache-fit".to_string(), "cache-exceed".to_string()]} platforms={vec!["native".to_string(), "sgx".to_string()]} />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
