use gloo_console::log;
use gloo_net::http::{Method, Request};
use time::OffsetDateTime;
use web_sys::{HtmlInputElement, HtmlOptionElement, HtmlSelectElement};
use yew::prelude::*;
use yewdux::prelude::*;
use yewdux_input::{Checkbox, InputDispatch};

use common::data_types::{
    describe_ui_element, Algorithm, Dataset, ExperimentType, Job, Measurement, Parameter, Platform,
    ProfilingConfiguration, VariantNames, EPC_SIZE_KB,
};
use std::collections::HashSet;
use std::str::FromStr;

use crate::components::{
    bs_popover::BsInitPopovers,
    checkbox::{CheckboxData, InputCheckbox, InputCheckboxes},
    number::InputNumber,
    select::{InputSelect, SelectDataOption},
    InfoPopover,
};
use crate::job_results_view::JobResultsView;
use crate::modal::Modal;
use crate::navigation::Navigation;
use crate::queue::QueueState;
use common::commit::CommitState;

use wasm_bindgen::JsCast;

fn create_popover(body: &str) -> InfoPopover {
    InfoPopover::new("".to_owned(), body.to_owned())
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
    let algs = Algorithm::VARIANTS;
    let exps = ExperimentType::VARIANTS;
    let params = Parameter::VARIANTS;
    let measurements = Measurement::VARIANTS;
    let platforms = Platform::VARIANTS;
    let datasets = Dataset::VARIANTS;
    let commit_store = use_store_value::<CommitState>();
    let algs = {
        // TODO This code is repeated in commits.rs for the baseline field.
        let mut algs = SelectDataOption::options_vec(algs);
        let found = algs.iter_mut().find(|o| o.value == "Latest Operator");
        if found.is_some() {
            let o: &mut SelectDataOption = found.unwrap(); // Putting &mut in front of the variable does not work. Type just to understand.
            if let Some(c) = commit_store.get_latest() {
                o.label = format!("Latest Operator ({})", c.get_title()).to_string();
                // We could put the id of the commit in the value field to offer not just the latest commit, but all.
            } else {
                o.enabled = false;
            }
        };
        algs
    };
    let alg_popover = create_popover(describe_ui_element("Algorithm"));
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
                        let id = match commit_store.get_latest() {
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
            store.algorithms = selected;
        })
    };
    let exps = SelectDataOption::options_vec(&exps);
    let exps_popover = create_popover(describe_ui_element("ExperimentType"));
    let exps_onchange = {
        let (_store, dispatch) = use_store::<ProfilingConfiguration>();
        dispatch.reduce_mut_callback_with(|store, e: Event| {
            let select_elem = e.target_unchecked_into::<HtmlSelectElement>();
            let value = select_elem.value();
            store.experiment_type = ExperimentType::from_str(&value).unwrap();
        })
    };
    let params = SelectDataOption::options_vec(&params);
    let params_popover = create_popover(describe_ui_element("Parameter"));
    let params_onchange = {
        let (_store, dispatch) = use_store::<ProfilingConfiguration>();
        dispatch.reduce_mut_callback_with(|store, e: Event| {
            let select_elem = e.target_unchecked_into::<HtmlSelectElement>();
            let value = select_elem.value();
            let param = Parameter::from_str(&value).unwrap();
            store.parameter = param.clone();
            let custom_size = Dataset::CustomSize {
                x: (EPC_SIZE_KB / 1024),
                y: (EPC_SIZE_KB / 1024),
            }; // It's terrible, but I'm using 0 as indicator that this value is a dummy value. to_teebench_cmd checks for zeros.
            if Parameter::OuterTableSize == param {
                store.datasets = HashSet::from([custom_size]);
            } else {
                store.datasets.remove(&custom_size);
            }
        })
    };
    let measurements = SelectDataOption::options_vec(&measurements);
    let measurements_popover = create_popover(describe_ui_element("Measurement"));
    let measurements_onchange = {
        let (_store, dispatch) = use_store::<ProfilingConfiguration>();
        dispatch.reduce_mut_callback_with(|store, e: Event| {
            let select_elem = e.target_unchecked_into::<HtmlSelectElement>();
            let value = select_elem.value();
            store.measurement = Measurement::from_str(&value).unwrap();
        })
    };
    let min_popover = create_popover(describe_ui_element("Min"));
    let min_onchange = {
        let (_store, dispatch) = use_store::<ProfilingConfiguration>();
        dispatch.reduce_mut_callback_with(|store, e: Event| {
            let input_num = e.target_unchecked_into::<HtmlInputElement>();
            let value = input_num.value();
            match store.parameter {
                Parameter::Threads | Parameter::JoinSelectivity | Parameter::OuterTableSize => {
                    // TODO Notify user of wrong input if unwrap fails.
                    let _value = i64::from_str_radix(&value, 10).unwrap();
                }
                Parameter::DataSkew => {
                    let _value = f64::from_str(&value).unwrap();
                }
                Parameter::Algorithms => {
                    // I guess do nothing?
                }
            }
            store.min = value;
        })
    };
    let max_popover = create_popover(describe_ui_element("Max"));
    let max_onchange = {
        let (_store, dispatch) = use_store::<ProfilingConfiguration>();
        dispatch.reduce_mut_callback_with(|store, e: Event| {
            let input_num = e.target_unchecked_into::<HtmlInputElement>();
            let value = input_num.value();
            match store.parameter {
                Parameter::Threads | Parameter::JoinSelectivity | Parameter::OuterTableSize => {
                    // TODO Notify user of wrong input if unwrap fails.
                    let _value = i64::from_str_radix(&value, 10).unwrap();
                }
                Parameter::DataSkew => {
                    let _value = f64::from_str(&value).unwrap();
                }
                Parameter::Algorithms => {
                    // I guess do nothing?
                }
            }
            store.max = value;
        })
    };
    let step_popover = create_popover(describe_ui_element("Step"));
    let step_onchange = {
        let (_store, dispatch) = use_store::<ProfilingConfiguration>();
        dispatch.reduce_mut_callback_with(|store, e: Event| {
            let input_num = e.target_unchecked_into::<HtmlInputElement>();
            let value = input_num.value();
            match store.parameter {
                Parameter::Threads | Parameter::JoinSelectivity | Parameter::OuterTableSize => {
                    // TODO Notify user of wrong input if unwrap fails.
                    let _value = i64::from_str_radix(&value, 10).unwrap();
                }
                Parameter::DataSkew => {
                    let _value = f64::from_str(&value).unwrap();
                }
                Parameter::Algorithms => {
                    // I guess do nothing?
                }
            }
            store.step = value;
        })
    };
    let datasets: Vec<CheckboxData> = datasets
        .iter()
        //.filter(|&d| d != &"Custom Size")
        .map(|d| {
            if d == &"Custom Size" {
                CheckboxData::new(&d, &d, true)
            } else {
                CheckboxData::new(&d, &d, false)
            }
        })
        .collect();
    let datasets_popover = create_popover(describe_ui_element("Datasets"));
    let datasets_onchange = {
        let (_store, dispatch) = use_store::<ProfilingConfiguration>();
        dispatch.reduce_mut_callback_with(|s, e: Event| {
            let input_check = e.target_unchecked_into::<HtmlInputElement>();
            let value = input_check.value();
            let bool_val = input_check.checked();
            let ds = Dataset::from_str(&value).unwrap();
            if bool_val {
                s.datasets.insert(ds);
            } else {
                s.datasets.remove(&ds);
            }
        })
    };
    let platforms: Vec<CheckboxData> = platforms
        .iter()
        .map(|p| CheckboxData::new(&p, &p, false))
        .collect();
    let platforms_popover = create_popover(describe_ui_element("Platforms"));
    let platforms_onchange = {
        let (_store, dispatch) = use_store::<ProfilingConfiguration>();
        dispatch.reduce_mut_callback_with(|s, e: Event| {
            let input_check = e.target_unchecked_into::<HtmlInputElement>();
            let value = input_check.value();
            let bool_val = input_check.checked();
            let pl = Platform::from_str(&value).unwrap();
            if bool_val {
                s.platforms.insert(pl);
            } else {
                s.platforms.remove(&pl);
            }
        })
    };
    let sort_popover = create_popover(describe_ui_element("Pre-Sort Data"));
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
            // Using different notation for clone call because otherwise Rc::clone is called.
            let store = ProfilingConfiguration::clone(&store);
            Box::pin(async move {
                //e.prevent_default(); // Doesn't seem to work with type="submit".
                use common::data_types::JobConfig;
                let job = Job::new(JobConfig::Profiling(store), OffsetDateTime::now_utc());
                let resp = Request::get("/api/job")
                    .method(Method::POST)
                    .json(&job)
                    .unwrap() // This should be impossible to fail.
                    .send()
                    .await
                    .expect("Server didn't respond. Is it running?");
                log!("Sent request got: ", format!("{resp:?}"));
                s.queue.push_back(job);
            })
        })
    };
    let (store, _dispatch) = use_store::<ProfilingConfiguration>();
    // dispatch.reduce_mut(|s| {
    //     // Instead of changing the form, I think it's better to keep the previous Custom settings (would be even better if that also applied to algorithms)
    //     s.set_preconfigured_experiment();
    // });
    let disable_controls = store.experiment_type != ExperimentType::Custom;
    let disable_submit = store.datasets.is_empty();
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
                            <form id="tbw-profiling-form" class="row g-3" method="get">
                                <div class="col-md">
                                    <div class="row g-3">
                                        <div id="tbw-profiling-form-algs" class="col-md">
                                            <InputSelect options={algs} onchange={algs_onchange} label={"Algorithm (select multiple)"} multiple={true} selected={store.algorithms.iter().map(|a| a.to_string()).collect::<Vec<_>>()} disabled={false} info_popover={alg_popover} />
                                        </div>
                                        <div id="tbw-profiling-form-experiment" class="col-md">
                                            <InputSelect options={exps} onchange={exps_onchange} label={"Experiment"} multiple={false} selected={vec![store.experiment_type.to_string()]} disabled={false} info_popover={exps_popover} />
                                        </div>
                                    </div>
                                    <div class="row g-3">
                                        <div id="tbw-profiling-form-measurement" class="col-md">
                                            <InputSelect options={measurements} onchange={measurements_onchange} label={"Measurement (Y-axis)"} multiple={false} selected={vec![store.measurement.to_string()]} disabled={disable_controls} info_popover={measurements_popover} />
                                            <InputSelect options={params} onchange={params_onchange} label={"Parameter (X-axis)"} multiple={false} selected={vec![store.parameter.to_string()]} disabled={disable_controls} info_popover={params_popover} />
                                        </div>
                                        <div id="tbw-profiling-form-values" class="col-md tbw-profiling-form-values">
                                            <InputNumber label={"min"} onchange={min_onchange} selected={store.min.to_string()} disabled={disable_controls} info_popover={min_popover} />
                                            <InputNumber label={"max"} onchange={max_onchange} selected={store.max.to_string()} disabled={disable_controls} info_popover={max_popover} />
                                            <InputNumber label={"step"} onchange={step_onchange} selected={store.step.to_string()} disabled={disable_controls} info_popover={step_popover} />
                                        </div>
                                    </div>
                                    <div class="row g-3">
                                        <div id="tbw-profiling-form-dataset" class="col-md">
                                            <InputCheckboxes title={"Dataset"} data={datasets} onchange={datasets_onchange} selected={store.datasets.iter().map(|ds| ds.to_string()).collect::<Vec<_>>()} disabled={disable_controls} info_popover={datasets_popover} />
                                        </div>
                                        <div id="tbw-profiling-form-platform" class="col-md">
                                            <InputCheckboxes title={"Platform"} data={platforms} onchange={platforms_onchange} selected={store.platforms.iter().map(|pl| pl.to_string()).collect::<Vec<_>>()} disabled={disable_controls} info_popover={platforms_popover} />
                                        </div>
                                        <div class="col-md">
                                            <fieldset class="row mb-3 col-md">
                                                <legend class="form-label invisible">{"Pre-sort data"}</legend>
                                                <InputCheckbox label={"Pre-sort data"} onchange={sort_onchange} value={"sort_data".to_string()} selected={store.sort_data} disabled={disable_controls} info_popover={sort_popover} />
                                            </fieldset>
                                        </div>
                                    </div>
                                    <button id="tbw-profiling-form-run" class="btn btn-primary" type="button" onclick={onsubmit} disabled={disable_submit} >{"Run experiment"}</button>
                                </div>
                            </form>
                            <JobResultsView />
                        </div>
                    </main>
                </div>
            </div>
            <Modal />
            <BsInitPopovers />
        </div>
    }
}
