use common::data_types::{
    Algorithm, Dataset, ExperimentChart, ExperimentType, JobConfig, Measurement, Parameter,
    Platform, TeebenchArgs,
};
// use gloo_console::log;
use common::commit::CommitState;
use js_sys::Object;
use serde_json::json;
use std::collections::HashMap;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;
use yewdux::prelude::*;

#[allow(non_snake_case)]
#[wasm_bindgen(module = "/deps/MyChart.js")]
extern "C" {
    #[derive(Clone, PartialEq, Default)]
    type MyChart;

    #[wasm_bindgen(constructor)]
    fn new() -> MyChart;

    #[wasm_bindgen(method)]
    fn draw(this: &MyChart, context: Object, config: &str);

    #[wasm_bindgen(method)]
    fn destroy(this: &MyChart);

    pub fn hljs_highlight(code: String) -> String;
}

//const COLORS: [&str; 2] = ["#de3d82", "#72e06a"]; // Original colors
const COLORS: [&str; 15] = [
    "#359B73", "#f0e442", "#000000", "#2271B2", "#AA0DB4", "#FF54ED", "#F748A5", "#00B19F",
    "#EB057A", "#d55e00", "#F8071D", "#3DB7E9", "#e69f00", "#FF8D1A", "#9EFF37",
];

#[derive(Clone, PartialEq, Default, Store)]
pub struct ChartState(MyChart, bool);

/// Returns: chart_type, labels: Json: Vec<&str>, datasets: Json<Vec<Obj<>>>, plugins: Json<Vec<Obj<>>>, scales: Json<Vec<Obj<>>>, options: Json<Vec<Obj<>>>
pub fn predefined_throughput_exp(
    alg_a: String,
    alg_b: String,
    data_a: Vec<f64>,
    data_b: Vec<f64>,
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
    let datasets = json!([
        {
            "label": alg_a,
            "backgroundColor": "#de3d82",
            "data": data_a
        },
        {
            "label": alg_b,
            "backgroundColor": "#72e06a",
            "data": data_b

        }
    ]);
    let d = match d {
        Dataset::CacheFit => "Throughput Cache Fit",
        Dataset::CacheExceed => "Throughput Cache Exceed",
    };
    let plugins = json!({
        "title": {
            "display": true,
            "text": d,
            "font": {
                "size":60
            }
        }
    });
    let scales = json!({
        "x": {"ticks": {"font": {"size": 40}}},
        "y": {"ticks": {"font": {"size": 40}},
        "text": "Throughput [M rec/s]",
        }
    });
    (chart_type, labels, datasets, plugins, scales)
}

pub fn predefined_scalability_exp(
    alg_a: String,
    alg_b: String,
    data_a: Vec<f64>,
    data_b: Vec<f64>,
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
    let datasets = json!([
        {
            "label": alg_a,
            "data": data_a,
            "backgroundColor": "#de3d82",
            "borderColor": "#de3d82",
            "yAxisID": "y",
            "borderWidth":5
        },
        {
            "label": alg_b,
            "data": data_b,
            "backgroundColor": "#72e06a",
            "borderColor": "#72e06a",
            "yAxisID": "y",
            "borderWidth":5
        }
    ]);
    let title = match d {
        Dataset::CacheFit => "Scalability Cache Fit",
        Dataset::CacheExceed => "Scalability Cache Exceed",
    };
    let plugins = json!({
        "title": {
            "display": true,
            "text": title,
            "font": {
                "size": 60
            }
        }
    });
    let scales = json!({
        "x": {
            "ticks": {
                "font": {
                    "size": 40
                }
            }
        },
        "y": {
            "ticks": {
                "font": {
                    "size": 40
                },
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

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ChartProps {
    pub exp_chart: ExperimentChart,
}

#[function_component]
pub fn Chart(ChartProps { exp_chart }: &ChartProps) -> Html {
    let commit_store = use_store_value::<CommitState>();
    let exp_chart = exp_chart.clone();
    let canvas_ref = NodeRef::default();
    let move_canvas_ref = canvas_ref.clone();
    //let (store, dispatch) = use_store::<ChartState>();
    use_effect_with_deps(
        move |_| {
            let commit_store = commit_store.clone();
            let mut exp_chart = exp_chart.clone();
            let canvas_ref = move_canvas_ref.clone();
            let chart_type;
            let labels;
            let datasets;
            let plugins;
            let scales;
            let options;
            match exp_chart.config {
                JobConfig::Profiling(conf) => match conf.experiment_type {
                    ExperimentType::Custom => {
                        let mut heading;
                        let y_axis_text;
                        match conf.measurement {
                            Measurement::EpcPaging => {
                                chart_type = "bar";
                                heading = String::from("EPC Paging with varying ");
                                y_axis_text = "EPC Misses";
                            } // TODO Not actually supported yet.
                            Measurement::Throughput => {
                                chart_type = "line";
                                heading = String::from("Throughput with varying ");
                                y_axis_text = "Throughput [M rec/s]";
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
                        }
                        let steps: Vec<_> = conf.param_value_iter();
                        labels = json!(steps);
                        let mut data = HashMap::new();
                        exp_chart
                            .results
                            .sort_unstable_by(|(a, _), (b, _)| a.cmp(b));
                        for (args, result) in exp_chart.results {
                            let v = data
                                .entry((args.algorithm, args.app_name, args.dataset))
                                .or_insert(vec![]);
                            v.push(match conf.measurement {
                                Measurement::EpcPaging => result["throughput"].clone(), // TODO Add EPC Paging.
                                Measurement::Throughput => result["throughput"].clone(),
                            });
                        }
                        let mut datasets_prep = vec![];

                        for (((alg, platform, ds), data_value), color) in
                            data.iter().zip(COLORS.iter().cycle())
                        {
                            datasets_prep.push(json!({
                                "label": format!("{alg} on {platform} with dataset {ds}"),
                                "data": data_value,
                                "backgroundColor": color,
                                "borderColor": color,
                                "yAxisID": "y",
                                "borderWidth":5
                            }));
                        }
                        datasets = json!(datasets_prep);
                        plugins = json!({
                            "title": {
                                "display": true,
                                "text": heading,
                                "font": {
                                    "size": 60
                                }
                            }
                        });
                        scales = json!({
                            "x": {
                                "ticks": {
                                    "font": {
                                        "size": 40
                                    }
                                }
                            },
                            "y": {
                                "ticks": {
                                    "font": {
                                        "size": 40
                                    },
                                    "min": 0
                                },
                                "text": y_axis_text,
                                "type": "linear",
                                "display": true,
                                "position": "left"
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
                                "font": {"size":40}
                            }
                        });
                        scales = json!({
                            "x": {
                                "ticks": {"font": {"size":20}},
                                "title" : {
                                    "display": true,
                                    "text": "Size of R [MB]",
                                    "font": {
                                        "size": 25
                                    }
                                }
                            },
                            "y": {
                                "text": "Throughput [M rec/s]",
                                "type": "linear",
                                "display": true,
                                "position": "left",
                                "ticks": {"font": {"size": 20}},
                                "title" : {
                                    "display": true,
                                    "text": "Throughput [M rec/s]",
                                    "font": {
                                        "size": 25
                                    }
                                }
                            },
                            "y1": {
                                "type": "linear",
                                "display": true,
                                "position": "right",
                                "ticks": {"font": {"size": 20}},
                                // grid line settings
                                "grid": {
                                    "drawOnChartArea": false, // only want the grid lines for one axis to show up
                                },
                                "title" : {
                                    "display": true,
                                    "text": "EPC Misses",
                                    "font": {
                                        "size": 25
                                    }
                                }
                            }
                        });
                    }
                    ExperimentType::Throughput => {
                        // TODO Replace with the funciton call predefined_throughput_exp
                        chart_type = "bar";
                        labels = json!(["native", "sgx"]);
                        datasets = json!([
                            {
                                "label": "v2.1",
                                "backgroundColor": "#de3d82",
                                "data": [164.11,33.51]
                            },
                            {
                                "label": "v2.2",
                                "backgroundColor": "#72e06a",
                                "data": [180.11,39.51]

                            }
                        ]);
                        plugins = json!({
                            "title": {
                                "display": true,
                                "text": "Throughput cache-fit",
                                "font": {
                                    "size":60
                                }
                            }
                        });
                        scales = json!({
                            "x": {"ticks": {"font": {"size": 40}}},
                            "y": {"ticks": {"font": {"size": 40}},
                            "text": "Throughput [M rec/s]",
                            }
                        });
                    }
                    ExperimentType::Scalability => {
                        todo!();
                    }
                },
                JobConfig::PerfReport(pr_conf) => {
                    let alg_a = commit_store
                        .get_id(&pr_conf.id)
                        .expect("Performance Report Config has a nonexistent commit id!")
                        .title
                        .clone();
                    let alg_b;
                    if let common::data_types::Algorithm::Commit(ref id) = pr_conf.baseline {
                        alg_b = commit_store
                            .get_id(id)
                            .expect("Performance Report Config has a nonexistent commit id!")
                            .title
                            .clone();
                    } else {
                        alg_b = pr_conf.baseline.to_string();
                    }
                    match pr_conf.exp_type {
                        ExperimentType::Throughput => {
                            let data_a = {
                                let native = exp_chart
                                    .results
                                    .iter()
                                    .find(|&tuple| {
                                        tuple.0
                                            == TeebenchArgs::for_throughput(
                                                Algorithm::Commit(1), // Actual commit id is irrelevant for now because only one Operator with id is allowed per perf report.
                                                // TODO Making this the actual commit also leads to a problem: I can't get the id from the commandline (there it's always -a ___), so the experiment runner would have to know whether it is running a baseline or commit. But it's just a vector with commands, it doesn't know which of those is baseline or commit...
                                                Platform::Native,
                                                pr_conf.dataset,
                                            )
                                    })
                                    .unwrap()
                                    .1["throughput"]
                                    .parse()
                                    .unwrap();
                                let sgx = exp_chart
                                    .results
                                    .iter()
                                    .find(|&tuple| {
                                        tuple.0
                                            == TeebenchArgs::for_throughput(
                                                Algorithm::Commit(1),
                                                Platform::Sgx,
                                                pr_conf.dataset,
                                            )
                                    })
                                    .unwrap()
                                    .1["throughput"]
                                    .parse()
                                    .unwrap();
                                vec![native, sgx]
                            };
                            let data_b = {
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
                                    .1["throughput"]
                                    .parse()
                                    .unwrap();
                                let sgx = exp_chart
                                    .results
                                    .iter()
                                    .find(|&tuple| {
                                        tuple.0
                                            == TeebenchArgs::for_throughput(
                                                pr_conf.baseline,
                                                Platform::Sgx,
                                                pr_conf.dataset,
                                            )
                                    })
                                    .unwrap()
                                    .1["throughput"]
                                    .parse()
                                    .unwrap();
                                vec![native, sgx]
                            };
                            (chart_type, labels, datasets, plugins, scales) =
                                predefined_throughput_exp(
                                    alg_a,
                                    alg_b,
                                    data_a,
                                    data_b,
                                    pr_conf.dataset,
                                );
                        }
                        ExperimentType::Scalability => {
                            let data_a = {
                                let mut res = vec![];
                                for threads in 1..=8 {
                                    res.push(
                                        exp_chart
                                            .results
                                            .iter()
                                            .find(|&tuple| {
                                                tuple.0
                                                    == TeebenchArgs::for_scalability(
                                                        Algorithm::Commit(1),
                                                        pr_conf.dataset,
                                                        threads,
                                                    )
                                            })
                                            .unwrap()
                                            .1["throughput"]
                                            .parse()
                                            .unwrap(),
                                    );
                                }
                                res
                            };
                            let data_b = {
                                let mut res = vec![];
                                for threads in 1..=8 {
                                    res.push(
                                        exp_chart
                                            .results
                                            .iter()
                                            .find(|&tuple| {
                                                tuple.0
                                                    == TeebenchArgs::for_scalability(
                                                        pr_conf.baseline,
                                                        pr_conf.dataset,
                                                        threads,
                                                    )
                                            })
                                            .unwrap()
                                            .1["throughput"]
                                            .parse()
                                            .unwrap(),
                                    );
                                }
                                res
                            };
                            (chart_type, labels, datasets, plugins, scales) =
                                predefined_scalability_exp(
                                    alg_a,
                                    alg_b,
                                    data_a,
                                    data_b,
                                    pr_conf.dataset,
                                );
                        }
                        ExperimentType::EpcPaging => {
                            todo!();
                        }
                        ExperimentType::Custom => {
                            unreachable!();
                        }
                    }
                }
                JobConfig::Compile(_) => panic!("Not allowed here!"),
            }
            options = json!({
                "responsive": true,
                "plugins": plugins,
                "scales": scales,
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
        (),
    );
    html! {
        <div>
            <canvas ref={canvas_ref}></canvas>
        </div>
    }
}

/*
#[derive(Debug, Serialize, Deserialize)]
pub struct Dataset {
    pub label: String,
    pub data: Vec<f64>,
    pub borderColor: String,
    pub backgroundColor: String,
    pub order: i32,
    pub r#type: String,
    pub yAxisID: String,
    pub borderWidth: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub labels: Vec<String>,
    pub datasets: Vec<Dataset>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Legend {
    pub position: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Font {
    pub size: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Title {
    pub display: bool,
    pub text: String,
    pub font: Font,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Plugin {
    pub legend: Legend,
    pub title: Title,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ticks {
    pub font: Font,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Grid {
    pub drawOnChartArea: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Scale {
    pub text: Option<String>,
    pub r#type: String,
    pub display: bool,
    pub position: String,
    pub ticks: Ticks,
    pub grid: Option<Grid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Scales {
    pub x: Scale,
    pub y: Scale,
    pub y1: Option<Scale>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Options {
    // animation
    pub responsive: bool,
    pub plugins: Plugin,
    pub scales: Scales,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub r#type: String,
    pub data: Data,
    pub options: Options,
}
*/
