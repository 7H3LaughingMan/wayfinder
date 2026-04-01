use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(
        extends = Object,
        js_name = ObservablePoint,
        js_namespace = PIXI,
        typescript_type = "PIXI.ObservablePoint"
    )]
    pub type JsObservablePoint;

    #[wasm_bindgen(method, getter)]
    pub fn x(this: &JsObservablePoint) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn y(this: &JsObservablePoint) -> f64;
}
