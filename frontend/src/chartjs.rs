use common::data_types::Report;
//use gloo_console::log;
use js_sys::Object;
use serde_json::json;
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

#[derive(Clone, PartialEq, Default, Store)]
pub struct ChartState(MyChart, bool);

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ChartProps {
    pub report: Report,
}

#[function_component]
pub fn Chart(ChartProps { report }: &ChartProps) -> Html {
    let report = report.clone();
    let canvas_ref = NodeRef::default();
    let move_canvas_ref = canvas_ref.clone();
    //let (store, dispatch) = use_store::<ChartState>();
    use_effect_with_deps(
        move |_| {
            let report = report.clone();
            let canvas_ref = move_canvas_ref.clone();
            let chart_type;
            let labels;
            let datasets;
            let plugins;
            let scales;
            let options;
            match report {
                Report::Epc => {
                    chart_type = "bar";
                    labels =
                        json!([8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120, 128]);
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
                    options = json!({
                        "responsive": true,
                        "plugins": plugins,
                        "scales": scales,
                    });
                }
                Report::EpcCht => {
                    chart_type = "bar";
                    labels =
                        json!([8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120, 128]);
                    datasets = json!([
                        {
                          "label": "Throughput",
                          "data": [64.22,26.54,23.16,22.0,21.32,19.29,16.59,15.68,13.61,12.65,12.07,1.37,0.97,1.0,1.04,1.15],
                          "borderColor": "#de3d82",
                          "backgroundColor": "#de3d82",
                          "order": 0,
                          "type": "line",
                          "yAxisID": "y",
                          "borderWidth":5
                        },
                        {
                          "label": "EPC Paging",
                          "data": [289860,293995,298271,302353,306725,318300,332876,341882,359938,372715,386494,1283342,1838011,1851572,1862953,1870652],
                          "borderColor": "#7e84fa",
                          "backgroundColor": "#7e84fa",
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
                            "text": "EPC Paging",
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
                    options = json!({
                        "responsive": true,
                        "plugins": plugins,
                        "scales": scales,
                    });
                }
                Report::Scalability => {
                    chart_type = "line";
                    labels = json!([1, 2, 3, 4, 5, 6, 7, 8]);
                    datasets = json!([
                        {
                            "label": "v2.1",
                            "data": [11.54,16.73,19.05,18.78,18.32,17.97,17.58,16.39],
                            "backgroundColor": "#de3d82",
                            "borderColor": "#de3d82",
                            "yAxisID": "y",
                            "borderWidth":5
                        },
                        {
                            "label": "v2.2",
                            "data": [12.84,17.03,21.05,21.78,19.32,18.54,18.01,17.09],
                            "backgroundColor": "#72e06a",
                            "borderColor": "#72e06a",
                            "yAxisID": "y",
                            "borderWidth":5
                        }
                    ]);
                    plugins = json!({
                        "title": {
                            "display": true,
                            "text": "Scalability cache-exceed",
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
                            "text": "Throughput [M rec/s]",
                            "type": "linear",
                            "display": true,
                            "position": "left"
                        }
                    });
                    options = json!({
                        "responsive": true,
                        "plugins": plugins,
                        "scales": scales,
                    });
                }
                Report::ScalabilityNativeSgxExample => {
                    chart_type = "line";
                    labels = json!([1, 2, 3, 4, 5, 6, 7, 8]);
                    datasets = json!([
                        {
                            "label": "Intel SGX",
                            "data": [15.11,21.94,15.52,13.82,12.19,7.77,7.1,6.26],
                            "backgroundColor": "#e6ab48",
                            "borderColor": "#e6ab48",
                            "yAxisID": "y",
                            "borderWidth":5
                        },
                        {
                            "label": "native CPU",
                            "data": [111.62,174.20,183.31,179.32,195.53,197.12,181.30,194.19],
                            "backgroundColor": "black",
                            "borderColor": "black",
                            "yAxisID": "y",
                            "borderWidth":5
                        }
                    ]);
                    plugins = json!({
                        "title": {
                            "display": true,
                            "text": "Scalability",
                            "font": {
                                "size": 40
                            }
                        }
                    });
                    scales = json!({
                        "x": {
                            "ticks": {
                                "font": {
                                    "size": 25
                                }
                            },
                            "title" : {
                                "display": true,
                                "text": "Threads",
                                "font": {
                                    "size": 25
                                }
                            }
                        },
                        "y": {
                            "ticks": {
                                "font": {
                                    "size": 25
                                },
                                "min": 0
                            },
                            "type": "linear",
                            "display": true,
                            "position": "left",
                            "title" : {
                                "display": true,
                                "text": "Throughput [M rec/s]",
                                "font": {
                                    "size": 25
                                }
                            }
                        }
                    });
                    options = json!({
                        "responsive": true,
                        "plugins": plugins,
                        "scales": scales,
                    });
                }
                Report::Throughput => {
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
                    options = json!({
                        "responsive": true,
                        "plugins": plugins,
                        "scales": scales,
                    });
                }
            }
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
            // let my_chart = if store.1 {
            //     store.0.clone()
            // } else {
            //     MyChart::new()
            // };
            let my_chart = MyChart::new();
            my_chart.draw(context, &config.to_string());

            // dispatch.reduce_mut(|state| {
            //     log!("async Chart dispatch running.");
            //     state.0 = my_chart.clone();
            //     state.1 = true;
            // });
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
