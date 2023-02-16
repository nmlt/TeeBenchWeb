use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct SelectDataOption {
    pub label: String,
    pub value: String,
}

impl SelectDataOption {
    pub fn new(label: String, value: String) -> Self {
        Self { label, value }
    }
    pub fn options_vec(options: &[&str]) -> Vec<Self> {
        options
            .iter()
            .map(|o| SelectDataOption::new(o.to_string(), o.to_string()))
            .collect()
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct InputSelectProps {
    pub options: Vec<SelectDataOption>,
    pub multiple: bool,
    pub label: String,
    pub onchange: Callback<Event>,
}

#[function_component]
pub fn InputSelect(
    InputSelectProps {
        options,
        multiple,
        label,
        onchange,
    }: &InputSelectProps,
) -> Html {
    let options = options
        .iter()
        .map(|o| html! { <option value={o.value.clone()}>{o.label.clone()}</option> });
    html! {
        <div>
            <label class="form-label" for={format!("select-{label}")}>{label.clone()}</label>
            <select class="form-select" id={format!("select-{label}")} type="select" multiple={*multiple} {onchange}>
                {for options}
            </select>
        </div>
    }
}
