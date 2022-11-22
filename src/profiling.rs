use yew::prelude::*;

use crate::navigation::Navigation;

use yewdux::prelude::*;
use yewdux_input::{Checkbox, InputDispatch};

#[derive(Store, Default, PartialEq, Clone)]
struct Form {
    sort_data: Checkbox,
    dataset: Option<String>,
}

#[derive(Clone, Properties, PartialEq)]
struct InputCheckboxProps {
    label: String,
}

#[function_component]
fn InputCheckbox(InputCheckboxProps { label }: &InputCheckboxProps) -> Html {
    let (store, dispatch) = use_store::<Form>();
    let onchange = dispatch.input_mut(|s, value| {
        s.sort_data = value;
    });

    html! {
        <div class="form-check">
            <input class="form-check-input" type="checkbox" {onchange} />
            <label class="form-check-label">{label.clone()}</label>
        </div>
    }
}

#[derive(Clone, PartialEq)]
struct RadioData {
    label: String,
    value: String,
}
impl RadioData {
    fn new(label: &str, value: &str) -> Self {
        RadioData {
            label: String::from(label),
            value: String::from(value),
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
struct InputRadioProps {
    data: Vec<RadioData>,
    title: String,
}

#[function_component]
fn InputRadio(InputRadioProps { data, title }: &InputRadioProps) -> Html {
    let (store, dispatch) = use_store::<Form>();
    let onchange = dispatch.input_mut(|s, value| {
        s.dataset = Some(value);
    });
    let options = data.iter().map(|RadioData {label, value}| {
        html!{
            <div class="form-check">
                <input class="form-check-input" onchange={onchange.clone()} type="radio" id={format!("radio-{}", value)} name="dataset" value={value.clone()} />
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

#[function_component(Profiling)]
pub fn profiling() -> Html {
    let datasets = vec![RadioData::new("cache-exceed", "cache-exceed"), RadioData::new("cache-fit", "cache-fit")];
    html!{
        <div>
            <h2>{"Profiling"}</h2>
            <Navigation />
            <form class="row g-3">
                <div class="col-md">
                    <InputCheckbox label={"Sort data"} />
                    <InputRadio data={datasets} title={"Dataset"}/>
                </div>
            </form>
        </div>
    }
}