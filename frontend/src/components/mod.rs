pub mod bs_popover;
pub mod checkbox;
pub mod collapse;
pub mod finding;
pub mod number;
pub mod select;
pub mod tag;
pub mod websocket;

use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct InfoPopover {
    pub title: String,
    pub body: String,
}

impl InfoPopover {
    pub fn new(title: String, body: String) -> Self {
        Self { title, body }
    }
    pub fn to_html(info_popover: &Option<InfoPopover>) -> Html {
        let help = if let Some(info_popover) = info_popover {
            if info_popover.title.is_empty() {
                html! {
                    <button type="button" class="btn btn-link"  data-bs-toggle="popover" data-bs-content={info_popover.body.clone()} data-bs-html="true" data-bs-trigger="hover">
                        <span class="bi-info-circle"></span>
                    </button>
                }
            } else {
                html! {
                    <button type="button" class="btn btn-link"  data-bs-toggle="popover" data-bs-title={info_popover.title.clone()} data-bs-content={info_popover.body.clone()} data-bs-html="true" data-bs-trigger="hover">
                        <span class="bi-info-circle"></span>
                    </button>
                }
            }
        } else {
            html! {}
        };
        help
    }
}
