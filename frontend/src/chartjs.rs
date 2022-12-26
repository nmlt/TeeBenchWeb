//#![allow(non_snake_case)]
use wasm_bindgen::prelude::wasm_bindgen;
//use wasm_bindgen::JsValue;
//use gloo_console::log;
//use serde::{Deserialize, Serialize};
use serde_json::json;

#[wasm_bindgen(module = "/deps/MyChart.js")]
extern "C" {
    pub type MyChart;

    #[wasm_bindgen(constructor)]
    pub fn new() -> MyChart;

    #[wasm_bindgen(method)]
    pub fn draw(this: &MyChart, element_id: &str, config: &str);
}

pub fn draw_chart(element_id: &str) {
    let my_chart = MyChart::new();
    let config = json!({
        "type": "bar",
        "data": {
            "labels": [8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120, 128],
            "datasets": [
                {
                    "label": "Throughput",
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
            ]
        },
        "options": {
            "responsive": true,
            "plugins": {
                "legend": {
                    "position": "top",
                },
                "title": {
                    "display": true,
                    "text": "EPC Paging v2.1",
                    "font": {"size":60}
                }
            },
            "scales": {
                "x":{"ticks": {"font":{"size":40}}},
                "y": {
                    "text": "Throughput [M rec/s]",
                    "type": "linear",
                    "display": true,
                    "position": "left",
                    "ticks": {"font":{"size":40}},
                },
                "y1": {
                    "type": "linear",
                    "display": true,
                    "position": "right",
                    "ticks": {"font":{"size":40}},
                    // grid line settings
                    "grid": {
                        "drawOnChartArea": false, // only want the grid lines for one axis to show up
                    },
                }
            }
        },
    });
    //log!(format!("{:?}", config.clone()));
    //let config = serde_wasm_bindgen::to_value(&config).unwrap();
    my_chart.draw(element_id, &config.to_string());
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
