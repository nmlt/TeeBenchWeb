use crate::components::InfoPopover;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct InputNumberProps {
    pub label: String,
    pub onchange: Callback<Event>,
    pub selected: String,
    pub disabled: bool,
    pub info_popover: Option<InfoPopover>,
}

#[function_component]
pub fn InputNumber(
    InputNumberProps {
        label,
        onchange,
        selected,
        disabled,
        info_popover,
    }: &InputNumberProps,
) -> Html {
    let help = InfoPopover::to_html(info_popover);
    html! {
        <div>
            <label class="form-label" for={format!("number-{label}")}>{label.clone()} {help}</label>
            <input class="form-control" {onchange} type="number" id={format!("number-{label}")} value={selected.clone()} disabled={*disabled} />
        </div>
    }
}
