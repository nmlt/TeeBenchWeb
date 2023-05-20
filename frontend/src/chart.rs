use anyhow::{bail, Result};
use gloo_console::log;
use rand::seq::SliceRandom;
use serde_json::json;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;
use yewdux::prelude::*;

use core::panic;
use std::collections::HashMap;

use common::commit::CommitState;
use common::data_types::{
    Algorithm, Dataset, ExperimentChart, ExperimentChartResult, ExperimentType, JobConfig,
    Measurement, Parameter, Platform, SingleRunResult, TeebenchArgs,
};

use crate::js_bindings::MyChart;

//const COLORS: [&str; 2] = ["#de3d82", "#72e06a"]; // Original colors
const COLORS: [&str; 15] = [
    "#359B73", "#f0e442", "#000000", "#2271B2", "#AA0DB4", "#FF54ED", "#F748A5", "#00B19F",
    "#EB057A", "#d55e00", "#F8071D", "#3DB7E9", "#e69f00", "#FF8D1A", "#9EFF37",
];

const COLORS2: [&str; 15] = [
    "#F08700", "#00A6A6", "#BBDEF0", "#462255", "#280004", "#F0FFCE", "#B7E3CC", "#9FB8AD",
    "#C8AD55", "#DCF2B0", "#FFE0B5", "#AA4465", "#5F5566", "#93E1D8", "#59F8E8",
];

fn get_color_by_algorithm(alg: &String) -> &str {
    let color_alg: HashMap<&str, &str> = HashMap::from([
        ("CHT", "#b44b20"),
        ("PHT", "#7b8b3d"),
        ("PSM", "#c7b186"),
        ("RHT", "#885a20"),
        ("RHO", "#e6ab48"),
        ("RSM", "#4c748a"),
        ("INL", "#7620b4"),
        ("MWAY", "#fd2455"),
        ("CRKJ", "#B3346C"),
    ]);
    let s = match color_alg.get(alg.as_str()) {
        // if not found - return a random color
        None => COLORS.choose(&mut rand::thread_rng()).unwrap().clone(),
        Some(c) => c,
    };
    s
}

fn get_measurement_from_single_result(
    single_run: &SingleRunResult,
    measurement: &Measurement,
) -> String {
    match measurement {
        Measurement::TotalEpcPaging => single_run.as_ref().map(|m| m["totalEWB"].clone()).unwrap(),
        Measurement::Throughput => single_run
            .as_ref()
            .map(|m| m["throughput"].clone())
            .unwrap(),
        Measurement::ThroughputAndTotalEPCPaging => panic!("Should not ask for a single value"),
        Measurement::Phase1Cycles => single_run
            .as_ref()
            .map(|m| m["phase1Cycles"].clone())
            .unwrap(),
        Measurement::Phase2Cycles => single_run
            .as_ref()
            .map(|m| m["phase2Cycles"].clone())
            .unwrap(),
        Measurement::TotalCycles => single_run
            .as_ref()
            .map(|m| m["cyclesPerTuple"].clone())
            .unwrap(),
        Measurement::TotalL2HitRatio => single_run
            .as_ref()
            .map(|m| m["totalL2HitRatio"].clone())
            .unwrap(),
        Measurement::TotalL3HitRatio => single_run
            .as_ref()
            .map(|m| m["totalL3HitRatio"].clone())
            .unwrap(),
        Measurement::TotalL2CacheMisses => single_run
            .as_ref()
            .map(|m| m["totalL2CacheMisses"].clone())
            .unwrap(),
        Measurement::TotalL3CacheMisses => single_run
            .as_ref()
            .map(|m| m["totalL3CacheMisses"].clone())
            .unwrap(),
        Measurement::IPC => single_run.as_ref().map(|m| m["totalIPC"].clone()).unwrap(),
        Measurement::IR => single_run.as_ref().map(|m| m["totalIR"].clone()).unwrap(),
        Measurement::TotalVoluntaryCS => single_run
            .as_ref()
            .map(|m| m["totalVoluntaryCS"].clone())
            .unwrap(),
        Measurement::TotalInvoluntaryCS => single_run
            .as_ref()
            .map(|m| m["totalInvoluntaryCS"].clone())
            .unwrap(),
        Measurement::TotalUserCpuTime => single_run
            .as_ref()
            .map(|m| m["totalUserCpuTime"].clone())
            .unwrap(),
        Measurement::TotalSystemCpuTime => single_run
            .as_ref()
            .map(|m| m["totalSystemCpuTime"].clone())
            .unwrap(),
        Measurement::TwoPhasesCycles => panic!("Should not ask for a single value"),
    }
}

fn create_data_hashmap(
    results: &ExperimentChartResult,
    measurement: Measurement,
    parameter: Parameter,
) -> HashMap<(Algorithm, Platform, Dataset), Vec<(String, String)>> {
    let mut data = HashMap::new();
    for (args, result) in results {
        let v = data
            .entry((args.algorithm, args.app_name, args.dataset))
            .or_insert(vec![]);
        let p = match parameter {
            Parameter::Threads => result.as_ref().map(|m| m["threads"].clone()).unwrap(),
            Parameter::DataSkew => args.threads.to_string(),
            Parameter::JoinSelectivity => args.selectivity.to_string(),
            Parameter::Algorithms => result.as_ref().map(|m| m["algorithm"].clone()).unwrap(),
            Parameter::OuterTableSize => args.x.unwrap().to_string(),
        };
        let m = get_measurement_from_single_result(result, &measurement);
        v.push((p, m));
    }
    data
}

/// Returns: chart_type, labels: Json: Vec<&str>, datasets: Json<Vec<Obj<>>>, plugins: Json<Vec<Obj<>>>, scales: Json<Vec<Obj<>>>
pub fn predefined_throughput_exp(
    alg_titles: Vec<String>,
    alg_data: Vec<Vec<f64>>,
    d: Dataset,
) -> (
    &'static str,
    serde_json::Value,
    serde_json::Value,
    serde_json::Value,
    serde_json::Value,
) {
    let chart_type = "bar";
    let labels = json!(["native", "sgx"]);
    let mut dataset_prep = vec![];
    for ((title, data), color) in alg_titles.iter().zip(alg_data).zip(COLORS) {
        dataset_prep.push(json!({
            "label": title,
            "backgroundColor": color,
            "data": data,
        }));
    }
    let datasets = json!(dataset_prep);
    let d = match d {
        Dataset::CacheFit => "Throughput Cache Fit",
        Dataset::CacheExceed => "Throughput Cache Exceed",
        _ => panic!("Cannot handle custom dataset size in predefined experiment!"),
    };
    let plugins = json!({
        "title": {
            "display": true,
            "text": d,
        }
    });
    let scales = json!({
        "y": {
            "text": "Throughput [M rec/s]",
        }
    });
    (chart_type, labels, datasets, plugins, scales)
}

pub fn predefined_scalability_exp(
    alg_titles: Vec<String>,
    alg_data: Vec<Vec<f64>>,
    d: Dataset,
) -> (
    &'static str,
    serde_json::Value,
    serde_json::Value,
    serde_json::Value,
    serde_json::Value,
) {
    let chart_type = "line";
    let labels = json!([1, 2, 3, 4, 5, 6, 7, 8]);
    let mut dataset_prep = vec![];
    for ((title, data), color) in alg_titles.iter().zip(alg_data).zip(COLORS) {
        dataset_prep.push(json!({
            "label": title,
            "data": data,
            "backgroundColor": color,
            "borderColor": color,
            "yAxisID": "y",
            "borderWidth":5
        }))
    }
    let datasets = json!(dataset_prep);
    let title = match d {
        Dataset::CacheFit => "Scalability Cache Fit",
        Dataset::CacheExceed => "Scalability Cache Exceed",
        _ => panic!("Cannot handle custom dataset size in predefined experiment!"),
    };
    let plugins = json!({
        "title": {
            "display": true,
            "text": title,
        }
    });
    let scales = json!({
        "y": {
            "ticks": {
                "min": 0
            },
            "text": "Throughput [M rec/s]",
            "type": "linear",
            "display": true,
            "position": "left"
        }
    });
    (chart_type, labels, datasets, plugins, scales)
}

pub fn predefined_epc_paging_exp(
    alg_title: String,
    alg_data: Vec<f64>,
    alg_data2: Vec<f64>,
) -> (
    &'static str,
    serde_json::Value,
    serde_json::Value,
    serde_json::Value,
    serde_json::Value,
) {
    let chart_type = "line";
    let labels = json!([8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120, 128]);
    let datasets = json!([
        {
            "label": format!("{alg_title} Throughput"),
            "data": alg_data,
            "backgroundColor": COLORS[0],
            "borderColor": COLORS[0],
            "yAxisID": "y",
            "borderWidth": 5
        },
        {
            "label": format!("EPC Paging {alg_title} on SGX"),
            "data": alg_data2,
            "backgroundColor": COLORS2[0],
            "borderColor": COLORS2[0],
            "yAxisID": "y1",
            "order" : 1,
            "type" : "bar"
        },
    ]);
    let title = format!("EPC Paging {alg_title}");
    let plugins = json!({
        "title": {
            "display": true,
            "text": title,
        }
    });
    let scales = json!({
        "y": {
            "text": "Throughput [M rec/s]",
            "type": "linear",
            "display": true,
            "position": "left",
            "title" : {
                "display": true,
                "text": "Throughput [M rec/s]",
            }
        },
        "y1": {
            "type": "linear",
            "display": true,
            "position": "right",
            // grid line settings
            "grid": {
                "drawOnChartArea": false, // only want the grid lines for one axis to show up
            },
            "title" : {
                "display": true,
                "text": "EPC Misses",
            }
        }
    });
    (chart_type, labels, datasets, plugins, scales)
}

fn prepare_perf_report_chart(
    commit_store: std::rc::Rc<CommitState>,
    exp_chart: ExperimentChart,
) -> Result<(
    &'static str,
    serde_json::Value,
    serde_json::Value,
    serde_json::Value,
    serde_json::Value,
)> {
    let JobConfig::PerfReport(ref pr_conf) = exp_chart.config else {
        bail!("Wrong JobConfig variant passed to PerfReport function");
    };

    let chart_type;
    let labels;
    let datasets;
    let plugins;
    let scales;

    let mut alg_titles = vec![];
    alg_titles.push(
        commit_store
            .get_by_id(&pr_conf.id)
            .expect("Performance Report Config has a nonexistent commit id!")
            .title
            .clone(),
    );
    if let common::data_types::Algorithm::Commit(ref id) = pr_conf.baseline {
        alg_titles.push(
            commit_store
                .get_by_id(id)
                .expect("Performance Report Config has a nonexistent commit id!")
                .title
                .clone(),
        );
    } else {
        alg_titles.push(pr_conf.baseline.to_string());
    }
    match pr_conf.exp_type {
        ExperimentType::Throughput => {
            let mut alg_data = vec![];
            let native = exp_chart
                .results
                .iter()
                .find(|&tuple| {
                    tuple.0
                        == TeebenchArgs::for_throughput(
                            Algorithm::Commit(pr_conf.id),
                            Platform::Native,
                            pr_conf.dataset,
                        )
                })
                .unwrap()
                .1
                .clone();

            let native = match native {
                Ok(map) => map["throughput"].parse().unwrap(),
                Err(e) => bail!("Error parsing chart: {e}"),
            };
            let sgx = exp_chart
                .results
                .iter()
                .find(|&tuple| {
                    tuple.0
                        == TeebenchArgs::for_throughput(
                            Algorithm::Commit(pr_conf.id),
                            Platform::Sgx,
                            pr_conf.dataset,
                        )
                })
                .unwrap()
                .1
                .clone();
            let sgx = match sgx {
                Ok(map) => map["throughput"].parse().unwrap(),
                Err(e) => bail!("Error parsing chart: {e}"),
            };
            alg_data.push(vec![native, sgx]);
            alg_data.push({
                let native = exp_chart
                    .results
                    .iter()
                    .find(|&tuple| {
                        tuple.0
                            == TeebenchArgs::for_throughput(
                                pr_conf.baseline,
                                Platform::Native,
                                pr_conf.dataset,
                            )
                    })
                    .unwrap()
                    .1
                    .clone();
                let native = match native {
                    Ok(map) => map["throughput"].parse().unwrap(),
                    Err(e) => bail!("Error parsing chart: {e}"),
                };
                let sgx: f64 = exp_chart
                    .get_result_values(
                        "throughput",
                        Platform::Sgx,
                        pr_conf.dataset,
                        pr_conf.baseline,
                        None,
                        None,
                        None,
                    )
                    .unwrap()
                    .into_iter()
                    .next()
                    .unwrap();
                vec![native, sgx]
            });
            (chart_type, labels, datasets, plugins, scales) =
                predefined_throughput_exp(alg_titles, alg_data, pr_conf.dataset);
        }
        ExperimentType::Scalability => {
            let mut alg_data = vec![];
            alg_data.push({
                let mut res = vec![];
                for threads in 1..=8 {
                    res.push(
                        exp_chart
                            .get_result_values(
                                "throughput",
                                Platform::Sgx,
                                pr_conf.dataset,
                                Algorithm::Commit(pr_conf.id),
                                Some(threads),
                                None,
                                None,
                            )
                            .unwrap()
                            .into_iter()
                            .next()
                            .unwrap(),
                    );
                }
                res
            });
            alg_data.push({
                let mut res = vec![];
                for threads in 1..=8 {
                    res.push(
                        exp_chart
                            .get_result_values(
                                "throughput",
                                Platform::Sgx,
                                pr_conf.dataset,
                                pr_conf.baseline,
                                Some(threads),
                                None,
                                None,
                            )
                            .unwrap()
                            .into_iter()
                            .next()
                            .unwrap(),
                    );
                }
                res
            });
            (chart_type, labels, datasets, plugins, scales) =
                predefined_scalability_exp(alg_titles, alg_data, pr_conf.dataset);
        }
        ExperimentType::EpcPaging => {
            let alg_title = alg_titles[1].clone();
            let mut alg_data = vec![];
            let mut alg_data2 = vec![];
            // 128 MB = 16_777_216
            let y_range: [u32; 1] = [128];
            // x (Relation R) starts at 8 MB, stepping each time 8 MB = 1_048_576
            log!(format!("exp chart results: {:#?}", exp_chart.results));
            for (x, &y) in (8..128).step_by(8).zip(y_range.iter().cycle()) {
                log!(format!(
                    "Searching for data: baseline: {:?} x {}, y {} ",
                    pr_conf.baseline, x, y
                ));
                let d1 = exp_chart
                    .get_result_values(
                        "throughput",
                        Platform::Sgx,
                        Dataset::new_custom(x, y),
                        pr_conf.baseline,
                        None,
                        Some(x),
                        Some(y),
                    )
                    .unwrap()
                    .into_iter()
                    .next()
                    .unwrap();
                let d2 = exp_chart
                    .get_result_values(
                        "totalEWB",
                        Platform::Sgx,
                        Dataset::new_custom(x, y),
                        pr_conf.baseline,
                        None,
                        Some(x),
                        Some(y),
                    )
                    .unwrap()
                    .into_iter()
                    .next()
                    .unwrap();
                alg_data.push(d1);
                alg_data2.push(d2);
            }
            (chart_type, labels, datasets, plugins, scales) =
                predefined_epc_paging_exp(alg_title, alg_data, alg_data2);
        }
        ExperimentType::Custom => {
            unreachable!();
        }
    }
    Ok((chart_type, labels, datasets, plugins, scales))
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ChartProps {
    pub exp_chart: ExperimentChart,
}

#[function_component]
pub fn Chart(ChartProps { exp_chart }: &ChartProps) -> Html {
    let commit_store = use_store_value::<CommitState>();
    //let title = format!("{exp_chart:#?}");
    let exp_chart = exp_chart.clone();
    let move_exp_chart = exp_chart.clone();
    let canvas_ref = NodeRef::default();
    let move_canvas_ref = canvas_ref.clone();
    use_effect_with_deps(
        move |_| {
            let commit_store = commit_store.clone();
            let mut exp_chart = move_exp_chart.clone();
            let canvas_ref = move_canvas_ref.clone();
            let mut chart_type;
            // let mut chart2_type;
            let labels;
            let datasets;
            let plugins;
            let scales;
            let options;
            let data;
            let data2;
            match exp_chart.config {
                JobConfig::Profiling(ref conf) => match conf.experiment_type {
                    ExperimentType::Custom => {
                        let mut heading;
                        let y_axis_text;
                        // let y2_axis_text;
                        match conf.measurement {
                            Measurement::TotalEpcPaging => {
                                chart_type = "bar";
                                heading = String::from("EPC Paging with varying ");
                                y_axis_text = "EPC Misses";
                            }
                            Measurement::Throughput => {
                                chart_type = "line";
                                heading = String::from("Throughput with varying ");
                                y_axis_text = "Throughput [M rec/s]";
                            }
                            Measurement::ThroughputAndTotalEPCPaging => {
                                chart_type = "line";
                                // chart2_type = "bar";
                                heading = String::from("Throughput and EPC Paging with varying ");
                                y_axis_text = "Throughput [M rec/s]";
                                // y2_axis_text = "EWB Misses"
                            }
                            Measurement::Phase1Cycles => {
                                chart_type = "bar";
                                heading = String::from("Phase 1 CPU cycles with varying ");
                                y_axis_text = "Cycles per tuple";
                            }
                            Measurement::Phase2Cycles => {
                                chart_type = "bar";
                                heading = String::from("Phase 2 CPU cycles with varying ");
                                y_axis_text = "Cycles per tuple";
                            }
                            Measurement::TotalCycles => {
                                chart_type = "bar";
                                heading = String::from("Total CPU cycleswith varying ");
                                y_axis_text = "Cycles per tuple";
                            }
                            Measurement::TotalL2HitRatio => {
                                chart_type = "line";
                                heading = String::from("Total L2 hit ratio with varying ");
                                y_axis_text = "L2 Hit Ratio";
                            }
                            Measurement::TotalL3HitRatio => {
                                chart_type = "line";
                                heading = String::from("Total L3 hit ratio with varying ");
                                y_axis_text = "L3 Hit Ratio";
                            }
                            Measurement::TotalL2CacheMisses => {
                                chart_type = "line";
                                heading = String::from("Total L2 cache misses with varying ");
                                y_axis_text = "L2 Cache Misses";
                            }
                            Measurement::TotalL3CacheMisses => {
                                chart_type = "line";
                                heading = String::from("Total L3 cache misses with varying ");
                                y_axis_text = "L3 Cache Misses";
                            }
                            Measurement::IPC => {
                                chart_type = "line";
                                heading = String::from("IPC with varying ");
                                y_axis_text = "IPC";
                            }
                            Measurement::IR => {
                                chart_type = "line";
                                heading = String::from("Instructions retired with varying ");
                                y_axis_text = "IR";
                            }
                            Measurement::TotalVoluntaryCS => {
                                chart_type = "line";
                                heading =
                                    String::from("Total voluntary context-switches with varying ");
                                y_axis_text = "Voluntary Context-switches";
                            }
                            Measurement::TotalInvoluntaryCS => {
                                chart_type = "line";
                                heading = String::from(
                                    "Total involuntary context-switches with varying ",
                                );
                                y_axis_text = "Involuntary context-switches";
                            }
                            Measurement::TotalUserCpuTime => {
                                chart_type = "line";
                                heading = String::from("User CPU time with varying ");
                                y_axis_text = "User CPU time [s]";
                            }
                            Measurement::TotalSystemCpuTime => {
                                chart_type = "line";
                                heading = String::from("System CPU time with varying ");
                                y_axis_text = "System CPU time [s]";
                            }
                            Measurement::TwoPhasesCycles => {
                                chart_type = "line";
                                heading = String::from("CPU cycles with varying ");
                                y_axis_text = "CPU Cycles / tuple";
                            }
                        }
                        match conf.parameter {
                            Parameter::Threads => {
                                heading.push_str("Threads");
                            }
                            Parameter::DataSkew => {
                                heading.push_str("Data Skew");
                            }
                            Parameter::JoinSelectivity => {
                                heading.push_str("Join Selectivity");
                            }
                            Parameter::Algorithms => {
                                chart_type = "bar";
                                heading.push_str("Algorithms");
                            }
                            Parameter::OuterTableSize => {
                                heading.push_str("Outer Table Size");
                            }
                        }
                        match conf.datasets.iter().next().unwrap() {
                            Dataset::CacheExceed => heading.push_str(" with dataset Cache Exceed"),
                            Dataset::CacheFit => heading.push_str(" with dataset Cache Fit"),
                            _ => (),
                        }
                        let steps: Vec<_> = conf.param_value_iter();
                        labels = json!(steps);
                        exp_chart // TODO Can this be moved to the top level of the function (do we need to do this always)?
                            .results
                            .sort_unstable_by(|(a, _), (b, _)| a.cmp(b));

                        let mut datasets_prep = vec![];

                        match conf.clone().measurement {
                            Measurement::ThroughputAndTotalEPCPaging => {
                                // data = create_data_hashmap(
                                //     &exp_chart.results,
                                //     Measurement::Throughput,
                                //     conf.clone().parameter,
                                // );
                                // data2 = create_data_hashmap(
                                //     &exp_chart.results,
                                //     Measurement::TotalEpcPaging,
                                //     conf.clone().parameter,
                                // );
                                scales = json!({
                                    "y": {
                                        "text": "Throughput [M rec/s]",
                                        "type": "linear",
                                        "display": true,
                                        "position": "left",
                                        "title" : {
                                            "display": true,
                                            "text": "Throughput [M rec/s]",
                                        }
                                    },
                                    "y1": {
                                        "type": "linear",
                                        "display": true,
                                        "position": "right",
                                        // grid line settings
                                        "grid": {
                                            "drawOnChartArea": false, // only want the grid lines for one axis to show up
                                        },
                                        "title" : {
                                            "display": true,
                                            "text": "EPC Misses",
                                        }
                                    }
                                });
                                // EPC paging
                                for a in conf.algorithms.iter() {
                                    let alg_color =
                                        get_color_by_algorithm(&a.to_string()).to_string().clone();
                                    let mut values: Vec<String> = vec![];
                                    for s in &steps {
                                        match exp_chart.results.iter().find(|(args, _res)| {
                                            args.algorithm.to_string() == a.to_string()
                                                && args.x.unwrap().to_string() == s.clone()
                                        }) {
                                            None => {}
                                            Some((_, r)) => {
                                                let val = get_measurement_from_single_result(
                                                    r,
                                                    &Measurement::TotalEpcPaging,
                                                );
                                                values.push(val);
                                            }
                                        }
                                    }
                                    datasets_prep.push(json!({
                                    "label": format!("{} EPC Paging", a.to_string()),
                                    "data": values,
                                    "backgroundColor": alg_color,
                                    "borderColor": alg_color,
                                    "yAxisID": "y1",
                                    "order": 1,
                                    "type": "bar"
                                    }));
                                }
                                //Throughput
                                for a in conf.algorithms.iter() {
                                    // let alg_color = get_color_by_algorithm(&a.to_string()).to_string().clone();
                                    let alg_color =
                                        COLORS.choose(&mut rand::thread_rng()).unwrap().clone();
                                    let mut values: Vec<String> = vec![];
                                    for s in &steps {
                                        match exp_chart.results.iter().find(|(args, _res)| {
                                            args.algorithm.to_string() == a.to_string()
                                                && args.x.unwrap().to_string() == s.clone()
                                        }) {
                                            None => {}
                                            Some((_, r)) => {
                                                let val = get_measurement_from_single_result(
                                                    r,
                                                    &Measurement::Throughput,
                                                );
                                                values.push(val);
                                            }
                                        }
                                    }
                                    datasets_prep.push(json!({
                                    "label": format!("Throughput {}", a.to_string()),
                                    "data": values,
                                    "backgroundColor": alg_color,
                                    "borderColor": alg_color,
                                    "yAxisID": "y",
                                    "order": 0
                                    }));
                                }
                            }
                            Measurement::TwoPhasesCycles => {
                                data = create_data_hashmap(
                                    &exp_chart.results,
                                    Measurement::Phase1Cycles,
                                    conf.clone().parameter,
                                );
                                data2 = create_data_hashmap(
                                    &exp_chart.results,
                                    Measurement::Phase2Cycles,
                                    conf.clone().parameter,
                                );
                                scales = json!({
                                    "x": {
                                        "stacked": true
                                    },
                                    "y": {
                                            "ticks": {
                                                "min": 0
                                            },
                                            "text": y_axis_text,
                                            "type": "linear",
                                            "display": true,
                                            "position": "left",
                                            "stacked": true
                                        }
                                });
                                for ((alg, platform, _dataset), data_value) in data.iter() {
                                    let alg = commit_store.get_title_by_algorithm(alg).unwrap();
                                    let alg_color = get_color_by_algorithm(&alg);
                                    // compare the global label (steps) with the data_value results
                                    // fill in with NULL if a data_value is missing, otherwise pass the result
                                    let mut values: Vec<String> = vec![];
                                    for s in &steps {
                                        let v = data_value.iter().find(|(x, _)| s == x);
                                        match v {
                                            None => values.push("NULL".to_string()),
                                            Some(val) => values.push(val.1.clone()),
                                        }
                                    }
                                    datasets_prep.push(json!({
                                        "label": format!("Phase 1 {alg} on {platform}"),
                                        "data": values,
                                        "backgroundColor": alg_color,
                                        "borderColor": alg_color,
                                        "yAxisID": "y",
                                        "borderWidth":5,
                                        "order": 0
                                    }));
                                }

                                for ((alg, platform, _dataset), data_value) in data2.iter() {
                                    let alg = commit_store.get_title_by_algorithm(alg).unwrap();
                                    let alg_color = get_color_by_algorithm(&alg).to_string()
                                        + &"AA".to_string();
                                    // compare the global label (steps) with the data_value results
                                    // fill in with NULL if a data_value is missing, otherwise pass the result
                                    let mut values: Vec<String> = vec![];
                                    for s in &steps {
                                        let v = data_value.iter().find(|(x, _)| s == x);
                                        match v {
                                            None => values.push("NULL".to_string()),
                                            Some(val) => values.push(val.1.clone()),
                                        }
                                    }
                                    datasets_prep.push(json!({
                                        "label": format!("Phase 2 {alg} on {platform}"),
                                        "data": values,
                                        "backgroundColor": alg_color,
                                        "borderColor": alg_color,
                                        "yAxisID": "y",
                                        "borderWidth":5,
                                        "order": 0
                                    }));
                                }
                            }
                            _ => {
                                data = create_data_hashmap(
                                    &exp_chart.results,
                                    conf.clone().measurement,
                                    conf.clone().parameter,
                                );
                                scales = json!({
                                    "y": {
                                            "ticks": {
                                                "min": 0
                                            },
                                            "text": y_axis_text,
                                            "type": "linear",
                                            "display": true,
                                            "position": "left"
                                        }
                                });
                                match conf.parameter {
                                    Parameter::Algorithms => {
                                        let mut values: Vec<String> = vec![];
                                        let mut colors: Vec<String> = vec![];
                                        for s in &steps {
                                            let alg_color =
                                                get_color_by_algorithm(s).to_string().clone();
                                            colors.push(alg_color);
                                            match exp_chart.results.iter().find(|(a, _)| {
                                                a.algorithm.to_string() == s.to_string()
                                            }) {
                                                None => {}
                                                Some((_, r)) => {
                                                    let val = get_measurement_from_single_result(
                                                        r,
                                                        &conf.measurement,
                                                    );
                                                    values.push(val);
                                                }
                                            }
                                        }
                                        datasets_prep.push(json!({
                                        "label": format!("Throughput"),
                                        "data": values,
                                        "backgroundColor": colors,
                                        "borderColor": colors,
                                        "yAxisID": "y",
                                        "borderWidth":5,
                                        "order": 0
                                        }));
                                    }
                                    _ => {
                                        for ((alg, platform, _dataset), data_value) in data.iter() {
                                            let alg =
                                                commit_store.get_title_by_algorithm(alg).unwrap();
                                            let alg_color = get_color_by_algorithm(&alg);
                                            // compare the global label (steps) with the data_value results
                                            // fill in with NULL if a data_value is missing, otherwise pass the result
                                            let mut values: Vec<String> = vec![];
                                            for s in &steps {
                                                let v = data_value.iter().find(|(x, _)| s == x);
                                                match v {
                                                    None => values.push("NULL".to_string()),
                                                    Some(val) => values.push(val.1.clone()),
                                                }
                                            }
                                            datasets_prep.push(json!({
                                                "label": format!("Throughput {alg} on {platform}"),
                                                "data": values,
                                                "backgroundColor": alg_color,
                                                "borderColor": alg_color,
                                                "yAxisID": "y",
                                                "borderWidth":5,
                                                "order": 0
                                            }));
                                        }
                                    }
                                }
                            }
                        }

                        datasets = json!(datasets_prep);
                        plugins = json!({
                            "title": {
                                "display": true,
                                "text": heading,
                            }
                        });
                    }
                    ExperimentType::EpcPaging => {
                        chart_type = "bar";
                        labels = json!([
                            8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120, 128
                        ]);
                        datasets = json!([
                            {
                                "label": "Throughput",
                                "labelString": "Throughput",
                                "display": true,
                                "data": [25.99, 20.36, 21.70, 20.60, 16.96, 14.00, 13.09, 17.80, 17.03, 18.26, 17.59, 16.86, 16.15, 14.71, 17.52, 17.55],
                                "borderColor": "#de3d82",
                                "backgroundColor": "#de3d82",
                                "border": 0,
                                "type": "line",
                                "yAxisID": "y",
                                "borderWidth": 5
                            },
                            {
                                "label": "EPC Paging",
                                "data": [325201, 334234, 338962, 343624, 348356, 353090, 357802, 362510, 367241, 371954, 376744, 381605, 386340, 390620, 394810, 399069],
                                "borderColor": "#7e84fa",
                                "backgroundColor": "#7e84fa",
                                // "type": "line",
                                "order": 1,
                                "yAxisID": "y1",
                            }
                        ]);
                        plugins = json!({
                            "legend": {
                                "position": "top",
                            },
                            "title": {
                                "display": true,
                                "text": "EPC Paging v2.1",
                            }
                        });
                        scales = json!({
                            "x": {
                                "title" : {
                                    "display": true,
                                    "text": "Size of R [MB]",
                                }
                            },
                            "y": {
                                "text": "Throughput [M rec/s]",
                                "type": "linear",
                                "display": true,
                                "position": "left",
                                "title" : {
                                    "display": true,
                                    "text": "Throughput [M rec/s]",
                                }
                            },
                            "y1": {
                                "type": "linear",
                                "display": true,
                                "position": "right",
                                // grid line settings
                                "grid": {
                                    "drawOnChartArea": false, // only want the grid lines for one axis to show up
                                },
                                "title" : {
                                    "display": true,
                                    "text": "EPC Misses",
                                }
                            }
                        });
                    }
                    ExperimentType::Throughput => {
                        // TODO TODO Do I have to do it the same way here as in Custom? Because I don't seem to need to do it that way in the PerfReport part. check that!
                        let data = create_data_hashmap(
                            &exp_chart.results,
                            conf.clone().measurement,
                            conf.clone().parameter,
                        );
                        let mut alg_titles = vec![];
                        let mut alg_data = vec![];
                        for ((alg, _platform, _dataset), value) in data {
                            alg_titles.push(commit_store.get_title_by_algorithm(&alg).unwrap());
                            alg_data.push(
                                value
                                    .iter()
                                    .map(|v| str::parse::<f64>(v.1.as_str()))
                                    .collect::<Result<Vec<_>, _>>()
                                    .unwrap(),
                            );
                        }
                        (chart_type, labels, datasets, plugins, scales) = predefined_throughput_exp(
                            alg_titles,
                            alg_data,
                            *conf.datasets.iter().next().unwrap(), // As this conf belongs to a `ExperimentChart` it can only have one dataset
                        );
                    }
                    ExperimentType::Scalability => {
                        todo!();
                    }
                },
                JobConfig::PerfReport(_) => {
                    (chart_type, labels, datasets, plugins, scales) =
                        match prepare_perf_report_chart(commit_store, exp_chart) {
                            Ok(tuple) => tuple,
                            Err(e) => panic!("Failed to create perf report chart: {e}"),
                        };
                }
                JobConfig::Compile(_) => panic!("Not allowed here!"),
            }
            options = json!({
                "responsive": true,
                "plugins": plugins,
                "scales": scales,
                "spanGaps": true
            });
            let config = json!({
                "type": chart_type,
                "data": {
                    "labels": labels,
                    "datasets": datasets,
                },
                "options": options,
            });
            let context = canvas_ref
                .cast::<HtmlCanvasElement>()
                .unwrap()
                .get_context("2d")
                .unwrap()
                .unwrap();
            let my_chart = MyChart::new();
            my_chart.draw(context, &config.to_string());

            move || my_chart.destroy()
        },
        exp_chart,
    );
    html! {
        // Looking at this bootstrap example which is also using chartjs: https://getbootstrap.com/docs/5.3/examples/dashboard/
        <canvas class="my-4 w-100" ref={canvas_ref}></canvas>
    }
}
