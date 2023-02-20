use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct InputCheckboxProps {
    pub label: String,
    pub onchange: Callback<Event>,
    pub value: String,
    pub selected: bool,
    pub disabled: bool,
}

#[function_component]
pub fn InputCheckbox(
    InputCheckboxProps {
        label,
        onchange,
        value,
        selected,
        disabled,
    }: &InputCheckboxProps,
) -> Html {
    html! {
        <div class="form-check">
            <input class="form-check-input" type="checkbox" {onchange} value={value.clone()} disabled={*disabled} checked={*selected} />
            <label class="form-check-label">{label.clone()}</label>
        </div>
    }
}

#[derive(Clone, PartialEq)]
pub struct CheckboxData {
    pub label: String,
    pub value: String,
}
impl CheckboxData {
    pub fn new(label: &str, value: &str) -> Self {
        Self {
            label: String::from(label),
            value: String::from(value),
        }
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct InputCheckboxesProps {
    pub title: String,
    pub data: Vec<CheckboxData>,
    pub onchange: Callback<Event>,
    pub selected: Vec<String>,
    pub disabled: bool,
}

#[function_component]
pub fn InputCheckboxes(
    InputCheckboxesProps {
        title,
        data,
        onchange,
        selected,
        disabled,
    }: &InputCheckboxesProps,
) -> Html {
    let options = data.iter().map(|CheckboxData { label, value}| {
        let selected = selected.contains(value);
        html! {
            <InputCheckbox label={label.clone()} onchange={onchange} disabled={*disabled} value={value.clone()} selected={selected} />
        }
    });
    html! {
        <fieldset class="row mb-3 col-md">
            <legend class="form-label">{title}</legend>
            <div class="col-sm-10">
                {for options}
            </div>
        </fieldset>
    }
}
