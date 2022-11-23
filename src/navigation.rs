use yew::prelude::*;
use yew_router::components::Link;

use crate::Route;

#[function_component(Navigation)]
pub fn navivation() -> Html {
    html! {
        <ul>
            <li><Link<Route> to={Route::Home}>{"Home"}</Link<Route>></li>
            <li><Link<Route> to={Route::Commits}>{"Commits"}</Link<Route>></li>
            <li><Link<Route> to={Route::Profiling}>{"Profiling"}</Link<Route>></li>
        </ul>
    }
    // let navigator = use_navigator().unwrap();

    // let go_home_button = {
    //     let navigator = navigator.clone();
    //     let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    //     html! {
    //         <button {onclick}>{"click to go home"}</button>
    //     }
    // };

    // let go_to_first_post_button = {
    //     let navigator = navigator.clone();
    //     let onclick = Callback::from(move |_| navigator.push(&Route::Post { id: "first-post".to_string() }));
    //     html! {
    //         <button {onclick}>{"click to go the first post"}</button>
    //     }
    // };

    // let go_to_secure_button = {
    //     let onclick = Callback::from(move |_| navigator.push(&Route::Secure));
    //     html! {
    //         <button {onclick}>{"click to go to secure"}</button>
    //     }
    // };

    // html! {
    //     <>
    //         {go_home_button}
    //         {go_to_first_post_button}
    //         {go_to_secure_button}
    //     </>
    // }
}
