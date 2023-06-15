use crate::js_bindings::bs_init_popovers;
use yew::prelude::*;

#[function_component]
pub fn BsInitPopovers() -> Html {
    use_effect(|| {
        bs_init_popovers();
    });
    html! {}
}
