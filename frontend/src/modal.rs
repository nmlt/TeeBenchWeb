use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Debug, Clone, PartialEq, Default, Store)]
pub struct ModalContent {
    pub content: Html,
}

#[function_component]
pub fn Modal() -> Html {
    let (content_store, _dispatch) = use_store::<ModalContent>();
    html! {
        <div class="modal fade" id="mainModal" tabindex="-1" aria-labelledby="mainModalLabel" aria-hidden="true">
            <div class="modal-dialog modal-dialog-scrollable modal-xl modal-fullscreen">
                {content_store.content.clone()}
            </div>
        </div>
    }
}
