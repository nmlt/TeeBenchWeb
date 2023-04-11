use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct CollapseProps {
    pub id: String,
    pub label: String,
    pub style: String,
    pub children: Children,
}

#[function_component]
pub fn Collapse(
    CollapseProps {
        id,
        label,
        style,
        children,
    }: &CollapseProps,
) -> Html {
    let class = classes!("btn", style);
    html! {
        <>
            <button classes={class} type="button" data-bs-toggle="collapse" data-bs-target={format!("#{id}")} aria-expanded="false" aria-controls={id.clone()}>
            {label}
            </button>
            <div class="collapse" id={id.clone()}>
                {for children.iter()}
            </div>
        </>
    }
}
