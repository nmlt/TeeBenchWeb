use gloo_console::log;
use gloo_net::http::{Method, Request};
use time::OffsetDateTime;
use web_sys::{HtmlInputElement, HtmlOptionElement, HtmlSelectElement};
use yew::prelude::*;
use yewdux::prelude::*;
use yewdux_input::{Checkbox, InputDispatch};

use common::data_types::{
    Algorithm, Dataset, ExperimentType, Job, Measurement, Parameter, Platform,
    ProfilingConfiguration, VariantNames,
};
use std::collections::HashSet;
use std::str::FromStr;

use crate::commits::CommitState;
use crate::components::{
    checkbox::{CheckboxData, InputCheckbox, InputCheckboxes},
    number::InputNumber,
    select::{InputSelect, SelectDataOption},
};
use crate::job_results_view::JobResultsView;
use crate::modal::Modal;
use crate::navigation::Navigation;
use crate::queue::QueueState;

use wasm_bindgen::JsCast;

// #[derive(Debug, Properties, Clone, PartialEq)]
// pub struct ProfilingMenuProps {
//     pub algs: Vec<String>,
//     pub exps: Vec<String>,
//     pub datasets: Vec<String>,
//     pub platforms: Vec<String>,
// }

#[function_component(Profiling)]
pub fn profiling() -> Html {
    let algs = Algorithm::VARIANTS;
    let exps = ExperimentType::VARIANTS;
    let params = Parameter::VARIANTS;
    let measurements = Measurement::VARIANTS;
    let platforms = Platform::VARIANTS;
    let datasets = Dataset::VARIANTS;
    let commit_store = use_store_value::<CommitState>();
    let algs = {
        let mut algs = SelectDataOption::options_vec(algs);
        let found = algs.iter_mut().find(|o| o.value == "Latest Operator");
        if found.is_some() {
            let o: &mut SelectDataOption = found.unwrap(); // Putting &mut in front of the variable does not work. Type just to understand.
            if let Some(c) = commit_store.0.first() {
                o.label = format!("Latest Operator ({})", c.title).to_string();
                // We could put the id of the commit in the value field to offer not just the latest commit, but all.
            } else {
                o.enabled = false;
            }
        };
        algs
    };
    let algs_onchange = {
        let (_store, dispatch) = use_store::<ProfilingConfiguration>();
        let commit_store = commit_store.clone();
        dispatch.reduce_mut_callback_with(move |store, e: Event| {
            let commit_store = commit_store.clone();
            let select_elem = e.target_unchecked_into::<HtmlSelectElement>();
            let html_collection = select_elem.selected_options();
            let mut selected = HashSet::new();
            for i in 0..html_collection.length() {
                let value = html_collection
                    .item(i)
                    .unwrap()
                    .dyn_into::<HtmlOptionElement>()
                    .unwrap();
                let value = Algorithm::from_str(&value.value()).unwrap();
                match value {
                    Algorithm::Commit(_) => {
                        let id = match commit_store.0.iter().max_by(|a, b| a.id.cmp(&b.id)) {
                            Some(c) => c.id,
                            None => {
                                log!("Error: No latest operator yet. Upload code in the Operator tab!");
                                panic!("This should not have been possible to happen :(");
                            }
                        };
                        selected.insert(Algorithm::Commit(id));
                    }
                    alg_variant => {
                        selected.insert(alg_variant);
                    }
                }
            }
            store.algorithm = selected;
        })
    };
    let exps = SelectDataOption::options_vec(&exps);
    let exps_onchange = {
        let (_store, dispatch) = use_store::<ProfilingConfiguration>();
        dispatch.reduce_mut_callback_with(|store, e: Event| {
            let select_elem = e.target_unchecked_into::<HtmlSelectElement>();
            let value = select_elem.value();
            store.experiment_type = ExperimentType::from_str(&value).unwrap();
        })
    };
    let params = SelectDataOption::options_vec(&params);
    let params_onchange = {
        let (_store, dispatch) = use_store::<ProfilingConfiguration>();
        dispatch.reduce_mut_callback_with(|store, e: Event| {
            let select_elem = e.target_unchecked_into::<HtmlSelectElement>();
            let value = select_elem.value();
            store.parameter = Parameter::from_str(&value).unwrap();
        })
    };
    let measurements = SelectDataOption::options_vec(&measurements);
    let measurements_onchange = {
        let (_store, dispatch) = use_store::<ProfilingConfiguration>();
        dispatch.reduce_mut_callback_with(|store, e: Event| {
            let select_elem = e.target_unchecked_into::<HtmlSelectElement>();
            let value = select_elem.value();
            store.measurement = Measurement::from_str(&value).unwrap();
        })
    };
    let min_onchange = {
        let (_store, dispatch) = use_store::<ProfilingConfiguration>();
        dispatch.reduce_mut_callback_with(|store, e: Event| {
            let input_num = e.target_unchecked_into::<HtmlInputElement>();
            let value = input_num.value();
            match store.parameter {
                Parameter::Threads | Parameter::JoinSelectivity => {
                    // TODO Notify user of wrong input if unwrap fails.
                    let _value = i64::from_str_radix(&value, 10).unwrap();
                }
                Parameter::DataSkew => {
                    let _value = f64::from_str(&value).unwrap();
                }
            }
            store.min = value;
        })
    };
    let max_onchange = {
        let (_store, dispatch) = use_store::<ProfilingConfiguration>();
        dispatch.reduce_mut_callback_with(|store, e: Event| {
            let input_num = e.target_unchecked_into::<HtmlInputElement>();
            let value = input_num.value();
            match store.parameter {
                Parameter::Threads | Parameter::JoinSelectivity => {
                    // TODO Notify user of wrong input if unwrap fails.
                    let _value = i64::from_str_radix(&value, 10).unwrap();
                }
                Parameter::DataSkew => {
                    let _value = f64::from_str(&value).unwrap();
                }
            }
            store.max = value;
        })
    };
    let step_onchange = {
        let (_store, dispatch) = use_store::<ProfilingConfiguration>();
        dispatch.reduce_mut_callback_with(|store, e: Event| {
            let input_num = e.target_unchecked_into::<HtmlInputElement>();
            let value = input_num.value();
            match store.parameter {
                Parameter::Threads | Parameter::JoinSelectivity => {
                    // TODO Notify user of wrong input if unwrap fails.
                    let _value = i64::from_str_radix(&value, 10).unwrap();
                }
                Parameter::DataSkew => {
                    let _value = f64::from_str(&value).unwrap();
                }
            }
            store.step = value;
        })
    };
    let datasets: Vec<CheckboxData> = datasets.iter().map(|d| CheckboxData::new(&d, &d)).collect();
    let datasets_onchange = {
        let (_store, dispatch) = use_store::<ProfilingConfiguration>();
        dispatch.reduce_mut_callback_with(|s, e: Event| {
            let input_check = e.target_unchecked_into::<HtmlInputElement>();
            let value = input_check.value();
            let bool_val = input_check.checked();
            let ds = Dataset::from_str(&value).unwrap();
            if bool_val {
                s.dataset.insert(ds);
            } else {
                s.dataset.remove(&ds);
            }
        })
    };
    let platforms: Vec<CheckboxData> = platforms
        .iter()
        .map(|p| CheckboxData::new(&p, &p))
        .collect();
    let platforms_onchange = {
        let (_store, dispatch) = use_store::<ProfilingConfiguration>();
        dispatch.reduce_mut_callback_with(|s, e: Event| {
            let input_check = e.target_unchecked_into::<HtmlInputElement>();
            let value = input_check.value();
            let bool_val = input_check.checked();
            let pl = Platform::from_str(&value).unwrap();
            if bool_val {
                s.platform.insert(pl);
            } else {
                s.platform.remove(&pl);
            }
        })
    };
    let sort_onchange = {
        let (_store, dispatch) = use_store::<ProfilingConfiguration>();
        dispatch.input_mut(|s, value: Checkbox| {
            s.sort_data = value.checked();
        })
    };
    let onsubmit = {
        // send some request to server
        let store = use_store_value::<ProfilingConfiguration>();
        let queue_dispatch = Dispatch::<QueueState>::new();
        queue_dispatch.reduce_mut_future_callback_with(move |s, _| {
            let store = ProfilingConfiguration::clone(&store);
            Box::pin(async move {
                //e.prevent_default(); // Doesn't seem to work with type="submit".
                let store = ProfilingConfiguration::clone(&store);
                let job = Job {
                    config: store.clone().into(),
                    submitted: OffsetDateTime::now_utc(),
                    ..Job::default()
                };
                let resp = Request::get("/api/job")
                    .method(Method::POST)
                    .json(&job)
                    .unwrap() // This should be impossible to fail.
                    .send()
                    .await
                    .expect("Server didn't respond. Is it running?");
                log!("Sent request got: ", format!("{resp:?}"));
                s.queue.push_back(store);
            })
        })
    };
    let (store, dispatch) = use_store::<ProfilingConfiguration>();
    dispatch.reduce_mut(|s| {
        s.set_preconfigured_experiment();
    });
    let disable_controls = store.experiment_type != ExperimentType::Custom;
    html! {
        <div class="container-fluid">
            <div class="row vh-100">
                <div class="col-12 col-sm-3 col-xl-2 px-sm-2 px-0 bg-dark d-flex sticky-top">
                    <Navigation active_nav_item={"Profiling"} />
                </div>
                <div class="col d-flex flex-column h-sm-100">
                    <main class="row">
                        <div class="col pt-4 col-lg-8">
                            <h2>{"Profiling"}</h2>
                            <form class="row g-3" method="get">
                                <div class="col-md">
                                    <div class="row g-3">
                                        <div class="col-md">
                                            <InputSelect options={algs} onchange={algs_onchange} label={"Algorithm (select multiple)"} multiple={true} selected={store.algorithm.iter().map(|a| a.to_string()).collect::<Vec<_>>()} disabled={false} />
                                        </div>
                                        <div class="col-md">
                                            <InputSelect options={exps} onchange={exps_onchange} label={"Experiment"} multiple={false} selected={vec![store.experiment_type.to_string()]} disabled={false} />
                                        </div>
                                    </div>
                                    <div class="row g-3">
                                        <div class="col-md">
                                            <InputSelect options={measurements} onchange={measurements_onchange} label={"Measurement (Y-axis)"} multiple={false} selected={vec![store.measurement.to_string()]} disabled={disable_controls} />
                                            <InputSelect options={params} onchange={params_onchange} label={"Parameter (X-axis)"} multiple={false} selected={vec![store.parameter.to_string()]} disabled={disable_controls} />
                                        </div>
                                        <div class="col-md">
                                            <InputNumber label={"min"} onchange={min_onchange} selected={store.min.to_string()} disabled={disable_controls} />
                                            <InputNumber label={"max"} onchange={max_onchange} selected={store.max.to_string()} disabled={disable_controls} />
                                            <InputNumber label={"step"} onchange={step_onchange} selected={store.step.to_string()} disabled={disable_controls} />
                                        </div>
                                    </div>
                                    <div class="row g-3">
                                        <div class="col-md">
                                            <InputCheckboxes title={"Dataset"} data={datasets} onchange={datasets_onchange} selected={store.dataset.iter().map(|ds| ds.to_string()).collect::<Vec<_>>()} disabled={disable_controls} />
                                        </div>
                                        <div class="col-md">
                                            <InputCheckboxes title={"Platform"} data={platforms} onchange={platforms_onchange} selected={store.platform.iter().map(|pl| pl.to_string()).collect::<Vec<_>>()} disabled={disable_controls} />
                                        </div>
                                        <div class="col-md">
                                            <fieldset class="row mb-3 col-md">
                                                <legend class="form-label invisible">{"Pre-sort data"}</legend>
                                                <InputCheckbox label={"Pre-sort data"} onchange={sort_onchange} value={"sort_data".to_string()} selected={store.sort_data} disabled={disable_controls} />
                                            </fieldset>
                                        </div>
                                    </div>
                                    <button class="btn btn-primary" type="button" onclick={onsubmit}>{"Run experiment"}</button>
                                </div>
                            </form>
                            <JobResultsView />
                        </div>
                    </main>
                </div>
            </div>
            <Modal />
        </div>
    }
}
