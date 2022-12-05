use gloo_console::log;
use gloo_net::http::{Method, Request};
use serde::{Deserialize, Serialize};
use serde_json::json;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlSelectElement;
use yew::prelude::*;
use yewdux::prelude::*;
use yewdux_input::{Checkbox, InputDispatch};

use crate::navigation::Navigation;

#[derive(Store, Default, PartialEq, Clone, Deserialize, Serialize)]
struct Form {
    algorithm: Option<String>,
    experiment_type: Option<String>,
    dataset: Option<String>,
    platform: Option<String>,
    sort_data: Checkbox,
}

#[derive(Clone, Debug, PartialEq)]
struct SelectDataOption {
    label: String,
    value: String,
}

impl SelectDataOption {
    fn new(label: String, value: String) -> Self {
        Self {
            label,
            value,
        }
    }
    fn options_vec(options: &Vec<String>) -> Vec<Self> {
        options.iter().map(|o| SelectDataOption::new(o.clone(), o.clone())).collect()
    }
}

#[derive(Clone, Properties, PartialEq)]
struct InputSelectProps {
    options: Vec<SelectDataOption>,
    onchange: Callback<Event>,
}

#[function_component]
fn InputSelect(InputSelectProps { options, onchange }: &InputSelectProps) -> Html {
    let options = options.iter().map(|o| html! { <option value={o.value.clone()}>{o.label.clone()}</option> });
    html! {
        <select class="form-select" type="select" {onchange}>
            {for options}
        </select>
    }
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
        html! {
            <div class="form-check">
                <input class="form-check-input" onchange={onchange.clone()} type="radio" id={format!("radio-{}", value)} name={title.clone()} value={value.clone()} />
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

// #[derive(Debug, Properties, Clone, PartialEq)]
// pub struct ProfilingMenuProps {
//     pub algs: Vec<String>,
//     pub exps: Vec<String>,
//     pub datasets: Vec<String>,
//     pub platforms: Vec<String>,
// }

#[function_component(Profiling)]
pub fn profiling() -> Html {
    let algs = vec!["Rho".to_owned(), "Cht".to_owned(),];
    let algs = SelectDataOption::options_vec(&algs);
    let algs_onchange = {
        let (store, dispatch) = use_store::<Form>();
        dispatch.reduce_mut_callback_with(|store, e: Event| {
            let select_elem = e.target_unchecked_into::<HtmlSelectElement>();
            let value = select_elem.value();
            store.algorithm = Some(value);
        })
    };
    let exps = vec!["EpcPaging".to_owned(),"Throughput".to_owned(),"CpuCyclesTuple".to_owned(),];
    let exps = SelectDataOption::options_vec(&exps);
    let exps_onchange = {
        let (store, dispatch) = use_store::<Form>();
        dispatch.reduce_mut_callback_with(|store, e: Event| {
            let select_elem = e.target_unchecked_into::<HtmlSelectElement>();
            let value = select_elem.value();
            store.experiment_type = Some(value);
        })
    };
    let datasets = vec!["CacheExceed".to_owned(), "CacheFit".to_owned()];
    let datasets: Vec<RadioData> = datasets.iter().map(|d| RadioData::new(&d, &d)).collect();
    let datasets_onchange = {
        let (_store, dispatch) = use_store::<Form>();
        dispatch.input_mut(|s, value| {
            s.dataset = Some(value);
        })
    };
    let platforms = vec!["Sgx".to_owned(), "Native".to_owned()];
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
        // send some request to server
        let (store, _dispatch) = use_store::<Form>();
        Callback::from(move |_| {
            //e.prevent_default(); // Doesn't seem to work with type="submit".
            let store = store.clone();
            spawn_local(async move {
                let resp = Request::get("/api/profiling")
                    .method(Method::POST)
                    .json(&store)
                    .unwrap()
                    .send()
                    .await
                    .unwrap();
            });
        })
    };
    html! {
        <div>
            <h2>{"Profiling"}</h2>
            <Navigation />
            <main>
                <form class="row g-3" method="get">
                    <div class="col-md">
                        <InputSelect options={algs} onchange={algs_onchange} />
                        <InputSelect options={exps} onchange={exps_onchange} />
                        <InputCheckbox label={"Pre-sort data"} onchange={sort_onchange} />
                        <InputRadio data={datasets} title={"Dataset"} onchange={datasets_onchange} />
                        <InputRadio data={platforms} title={"Platform"} onchange={platforms_onchange} />
                        <button class="btn btn-primary" type="button" onclick={onsubmit}>{"Run experiment"}</button>
                    </div>
                </form>
            </main>
        </div>
    }
}
