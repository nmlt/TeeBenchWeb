use js_sys::Object;
use wasm_bindgen::prelude::wasm_bindgen;

#[allow(non_snake_case)]
#[wasm_bindgen(module = "/deps/MyChart.js")]
extern "C" {
    #[derive(Clone, PartialEq, Default)]
    pub type MyChart;

    #[wasm_bindgen(constructor)]
    pub fn new() -> MyChart;

    #[wasm_bindgen(method)]
    pub fn draw(this: &MyChart, context: Object, config: &str);

    #[wasm_bindgen(method)]
    pub fn destroy(this: &MyChart);

    pub fn hljs_highlight(code: String) -> String;
}
