use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct TagProps {
    pub text: String,
}

#[function_component]
pub fn Tag(TagProps { text }: &TagProps) -> Html {
    html! {
        <span class="badge text-bg-secondary m-1">{text}</span>
    }
}
