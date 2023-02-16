use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct InputCheckboxProps {
    pub label: String,
    pub onchange: Callback<Event>,
}

#[function_component]
pub fn InputCheckbox(InputCheckboxProps { label, onchange }: &InputCheckboxProps) -> Html {
    html! {
        <div class="form-check">
            <input class="form-check-input" type="checkbox" {onchange} />
            <label class="form-check-label">{label.clone()}</label>
        </div>
    }
}
