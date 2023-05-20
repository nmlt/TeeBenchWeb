use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Debug, Clone, PartialEq, Default, Store)]
pub struct ModalContent {
    pub content: Html,
}

impl ModalContent {
    pub fn new(content: Html) -> Self {
        Self { content }
    }
    pub fn with_modal_skeleton(body: Html, title: Html) -> Self {
        Self {
            content: html! {
                <div class="modal-content">
                    <div class="modal-header">
                        <h5 class="modal-title">{title}</h5>
                        <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                    </div>
                    <div class="modal-body">
                        {body}
                    </div>
                    <div class="modal-footer">
                        <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{"Close"}</button>
                    </div>
                </div>
            },
        }
    }
}

#[function_component]
pub fn Modal() -> Html {
    let (content_store, _dispatch) = use_store::<ModalContent>();
    html! {
        <div class="modal fade" id="mainModal" tabindex="-1" aria-labelledby="mainModalLabel" aria-hidden="true">
            <div class="modal-dialog modal-dialog-scrollable modal-xl modal-fullscreen-xxl-down">
                {content_store.content.clone()}
            </div>
        </div>
    }
}
