use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct InputNumberProps {
    pub label: String,
    pub onchange: Callback<Event>,
}

#[function_component]
pub fn InputNumber(InputNumberProps { label, onchange }: &InputNumberProps) -> Html {
    html! {
        <div>
            <label class="form-label" for={format!("number-{label}")}>{label.clone()}</label>
            <input class="form-control" {onchange} type="number" id={format!("number-{label}")} />
        </div>
    }
}
