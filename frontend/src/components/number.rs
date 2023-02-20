use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct InputNumberProps {
    pub label: String,
    pub onchange: Callback<Event>,
    pub selected: String,
    pub disabled: bool,
}

#[function_component]
pub fn InputNumber(InputNumberProps { label, onchange, selected, disabled }: &InputNumberProps) -> Html {
    html! {
        <div>
            <label class="form-label" for={format!("number-{label}")}>{label.clone()}</label>
            <input class="form-control" {onchange} type="number" id={format!("number-{label}")} value={selected.clone()} disabled={*disabled} />
        </div>
    }
}
