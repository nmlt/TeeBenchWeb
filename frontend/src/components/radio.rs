use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct RadioData {
    pub label: String,
    pub value: String,
}
impl RadioData {
    pub fn new(label: &str, value: &str) -> Self {
        RadioData {
            label: String::from(label),
            value: String::from(value),
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct InputRadioProps {
    pub data: Vec<RadioData>,
    pub title: String,
    pub onchange: Callback<Event>,
}

#[function_component]
pub fn InputRadio(
    InputRadioProps {
        data,
        title,
        onchange,
    }: &InputRadioProps,
) -> Html {
    let options = data.iter().map(|RadioData {label, value}| {
        html! {
            <div class="form-check">
                <input class="form-check-input" onchange={onchange.clone()} type="checkbox" id={format!("radio-{}", value)} name={title.clone()} value={value.clone()} />
                <label class="form-label" for={format!("radio-{}", value)}>{label.clone()}</label>
            </div>
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
