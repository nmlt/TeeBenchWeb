use yew::prelude::*;

use super::InfoPopover;

#[derive(Clone, Properties, PartialEq)]
pub struct InputCheckboxProps {
    pub label: String,
    pub onchange: Callback<Event>,
    pub value: String,
    pub selected: bool,
    pub disabled: bool,
    pub info_popover: Option<InfoPopover>,
}

#[function_component]
pub fn InputCheckbox(
    InputCheckboxProps {
        label,
        onchange,
        value,
        selected,
        disabled,
        info_popover,
    }: &InputCheckboxProps,
) -> Html {
    let help = InfoPopover::to_html(info_popover);
    html! {
        <div class="form-check">
            <input class="form-check-input" type="checkbox" {onchange} value={value.clone()} disabled={*disabled} checked={*selected} />
            <label class="form-check-label">{label.clone()} {help}</label>
        </div>
    }
}

#[derive(Clone, PartialEq)]
pub struct CheckboxData {
    pub label: String,
    pub value: String,
    pub disabled: bool,
}
impl CheckboxData {
    pub fn new(label: &str, value: &str, disabled: bool) -> Self {
        Self {
            label: String::from(label),
            value: String::from(value),
            disabled,
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
    pub info_popover: Option<InfoPopover>,
}

#[function_component]
pub fn InputCheckboxes(
    InputCheckboxesProps {
        title,
        data,
        onchange,
        selected,
        disabled: whole_disabled,
        info_popover,
    }: &InputCheckboxesProps,
) -> Html {
    let options = data.iter().map(|CheckboxData { label, value, disabled }| {
        let selected = selected.contains(value);
        let disabled = *whole_disabled || *disabled;
        html! {
            <InputCheckbox label={label.clone()} onchange={onchange} disabled={disabled} value={value.clone()} selected={selected} />
        }
    });
    let help = InfoPopover::to_html(info_popover);
    html! {
        <fieldset class="row mb-3 col-md">
            <legend class="form-label">{title} {help}</legend>
            <div class="col-sm-10">
                {for options}
            </div>
        </fieldset>
    }
}
