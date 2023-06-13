use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct SelectDataOption {
    pub label: String,
    pub value: String,
    pub enabled: bool,
}

impl SelectDataOption {
    pub fn new(label: String, value: String, enabled: bool) -> Self {
        Self {
            label,
            value,
            enabled,
        }
    }
    pub fn options_vec(options: &[&str]) -> Vec<Self> {
        options
            .iter()
            .map(|o| SelectDataOption::new(o.to_string(), o.to_string(), true))
            .collect()
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct InputSelectProps {
    pub options: Vec<SelectDataOption>,
    pub multiple: bool,
    pub label: String,
    pub onchange: Callback<Event>,
    /// If None, the first in the options vector will be selected.
    pub selected: Vec<String>,
    pub disabled: bool,
}

#[function_component]
pub fn InputSelect(
    InputSelectProps {
        options,
        multiple,
        label,
        onchange,
        selected,
        disabled,
    }: &InputSelectProps,
) -> Html {
    let options = options
        .iter()
        .map(|o| {
            if selected.contains(&o.value) {
                return html! { <option value={o.value.clone()} selected={true} disabled={!o.enabled}>{o.label.clone()}</option> };
            }
            html! { <option value={o.value.clone()} disabled={!o.enabled}>{o.label.clone()}</option> }
        });
    let id: String = label.chars().filter(|c| c.is_alphanumeric()).collect();
    html! {
        <div>
            <label class="form-label" for={format!("select-{id}")}>{label.clone()}</label>
            <select class="form-select" id={format!("select-{id}")} type="select" multiple={*multiple} {onchange} disabled={*disabled} >
                {for options}
            </select>
        </div>
    }
}
