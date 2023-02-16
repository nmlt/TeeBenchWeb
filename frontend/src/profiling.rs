use gloo_console::log;
use gloo_net::http::{Method, Request};
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, HtmlOptionElement, HtmlSelectElement};
use yew::prelude::*;
use yewdux::prelude::*;
use yewdux_input::{Checkbox, InputDispatch};

use common::data_types::{
    Algorithm, Dataset, ExperimentType, Measurement, Parameter, Platform, ProfilingConfiguration,
    VariantNames,
};
use std::collections::HashSet;
use std::str::FromStr;

use crate::job_results_view::JobResultsView;
use crate::modal::Modal;
use crate::navigation::Navigation;
use crate::components::{
    select::{InputSelect, SelectDataOption},
    checkbox::InputCheckbox,
    radio::{InputRadio, RadioData},
    number::InputNumber,
};

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
    let algs = SelectDataOption::options_vec(algs);
    let algs_onchange = {
        let (_store, dispatch) = use_store::<ProfilingConfiguration>();
        dispatch.reduce_mut_callback_with(|store, e: Event| {
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
                    //                     Algorithm::Commit(_) => {
                    //                         // TODO Get latest commit id.
                    //                         let id = 0;
                    //                         selected.insert(Algorithm::Commit(id));
                    //                     }
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
            let value = i64::from_str_radix(&value, 10).unwrap();
            store.min = value;
        })
    };
    let max_onchange = {
        let (_store, dispatch) = use_store::<ProfilingConfiguration>();
        dispatch.reduce_mut_callback_with(|store, e: Event| {
            let input_num = e.target_unchecked_into::<HtmlInputElement>();
            let value = input_num.value();
            let value = i64::from_str_radix(&value, 10).unwrap();
            store.max = value;
        })
    };
    let step_onchange = {
        let (_store, dispatch) = use_store::<ProfilingConfiguration>();
        dispatch.reduce_mut_callback_with(|store, e: Event| {
            let input_num = e.target_unchecked_into::<HtmlInputElement>();
            let value = input_num.value();
            let value = i64::from_str_radix(&value, 10).unwrap();
            store.step = value;
        })
    };
    let datasets: Vec<RadioData> = datasets.iter().map(|d| RadioData::new(&d, &d)).collect();
    let datasets_onchange = {
        let (_store, dispatch) = use_store::<ProfilingConfiguration>();
        dispatch.input_mut(|s, value: String| {
            s.dataset = HashSet::from([Dataset::from_str(&value).unwrap()]);
        })
    };
    let platforms: Vec<RadioData> = platforms.iter().map(|p| RadioData::new(&p, &p)).collect();
    let platforms_onchange = {
        let (_store, dispatch) = use_store::<ProfilingConfiguration>();
        dispatch.input_mut(|s, value: String| {
            s.platform = HashSet::from([Platform::from_str(&value).unwrap()]);
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
        let (store, _dispatch) = use_store::<ProfilingConfiguration>();
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
                log!("Sent request got: ", format!("{resp:?}"));
            });
        })
    };
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
                                            <InputSelect options={algs} onchange={algs_onchange} label={"Algorithm (select multiple)"} multiple={true} />
                                        </div>
                                        <div class="col-md">
                                            <InputSelect options={exps} onchange={exps_onchange} label={"Experiment"} multiple={false} />
                                        </div>
                                    </div>
                                    <div class="row g-3">
                                        <div class="col-md">
                                            <InputSelect options={measurements} onchange={measurements_onchange} label={"Measurement (Y-axis)"} multiple={false} />
                                            <InputSelect options={params} onchange={params_onchange} label={"Parameter (X-axis)"} multiple={false} />
                                        </div>
                                        <div class="col-md">
                                            <InputNumber label={"min"} onchange={min_onchange} />
                                            <InputNumber label={"max"} onchange={max_onchange} />
                                            <InputNumber label={"step"} onchange={step_onchange} />
                                        </div>
                                    </div>
                                    <div class="row g-3">
                                        <div class="col-md">
                                            <InputRadio data={datasets} title={"Dataset"} onchange={datasets_onchange} />
                                        </div>
                                        <div class="col-md">
                                            <InputRadio data={platforms} title={"Platform"} onchange={platforms_onchange} />
                                        </div>
                                        <div class="col-md">
                                            <InputCheckbox label={"Pre-sort data"} onchange={sort_onchange} />
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
