use yew::prelude::*;
use yewdux::prelude::*;
use yewdux_input::{Checkbox, InputDispatch};

use crate::navigation::Navigation;

#[derive(Store, Default, PartialEq, Clone)]
struct Form {
    sort_data: Checkbox,
    dataset: Option<String>,
    platform: Option<String>,
}

#[derive(Clone, Properties, PartialEq)]
struct InputCheckboxProps {
    label: String,
    onchange: Callback<Event>,
}

#[function_component]
fn InputCheckbox(InputCheckboxProps { label, onchange }: &InputCheckboxProps) -> Html {
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
    onchange: Callback<Event>,
}

#[function_component]
fn InputRadio(
    InputRadioProps {
        data,
        title,
        onchange,
    }: &InputRadioProps,
) -> Html {
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

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct ProfilingMenuProps {
    pub algs: Vec<String>,
    pub exps: Vec<String>,
    pub datasets: Vec<String>,
    pub platforms: Vec<String>,
}

#[function_component(Profiling)]
pub fn profiling(
    ProfilingMenuProps {
        algs: _,
        exps: _,
        datasets,
        platforms,
    }: &ProfilingMenuProps,
) -> Html {
    let datasets: Vec<RadioData> = datasets.iter().map(|d| RadioData::new(&d, &d)).collect();
    let datasets_onchange = {
        let (_store, dispatch) = use_store::<Form>();
        dispatch.input_mut(|s, value| {
            s.dataset = Some(value);
        })
    };
    let platforms: Vec<RadioData> = platforms.iter().map(|p| RadioData::new(&p, &p)).collect();
    let platforms_onchange = {
        let (_store, dispatch) = use_store::<Form>();
        dispatch.input_mut(|s, value| {
            s.platform = Some(value);
        })
    };
    let sort_onchange = {
        let (_store, dispatch) = use_store::<Form>();
        dispatch.input_mut(|s, value| {
            s.sort_data = value;
        })
    };
    let onsubmit = {
        // preventdefault
        // send some request to server
    };
    html! {
        <div>
            <h2>{"Profiling"}</h2>
            <Navigation />
            <main>
                <form class="row g-3" method="get">
                    <div class="col-md">
                        <InputCheckbox label={"Sort data"} onchange={sort_onchange} />
                        <InputRadio data={datasets} title={"Dataset"} onchange={datasets_onchange} />
                        <InputRadio data={platforms} title={"Platform"} onchange={platforms_onchange} />
                        <button class="btn btn-primary" type="submit">{"Run experiment"}</button>
                    </div>
                </form>
            </main>
        </div>
    }
}
