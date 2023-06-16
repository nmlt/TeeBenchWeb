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

#[derive(Debug, Clone, PartialEq)]
pub struct InfoPopover {
    pub title: String,
    pub body: String,
}

impl InfoPopover {
    pub fn new(title: String, body: String) -> Self {
        Self { title, body }
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
    pub info_popover: Option<InfoPopover>,
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
        info_popover,
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
    html! {
        <div>
            <label class="form-label" for={format!("select-{id}")}>{label.clone()} {help}</label>
            <select class="form-select" id={format!("select-{id}")} type="select" multiple={*multiple} {onchange} disabled={*disabled} >
                {for options}
            </select>
        </div>
    }
}
