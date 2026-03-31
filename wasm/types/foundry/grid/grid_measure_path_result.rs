use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(typescript_type = "foundry.grid.GridMeasurePathResult")]
    pub type JsGridMeasurePathResult;

    #[wasm_bindgen(method, getter)]
    pub fn distance(this: &JsGridMeasurePathResult) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn cost(this: &JsGridMeasurePathResult) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn spaces(this: &JsGridMeasurePathResult) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn diagonals(this: &JsGridMeasurePathResult) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn euclidean(this: &JsGridMeasurePathResult) -> f64;
}
