use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = Object,
        js_name = Matrix,
        js_namespace = PIXI,
        typescript_type = "PIXI.Matrix"
    )]
    pub type JsMatrix;

    #[wasm_bindgen(constructor, js_class = Matrix, js_namespace = PIXI)]
    pub fn new(a: f64, b: f64, c: f64, d: f64, tx: f64, ty: f64) -> JsMatrix;

    #[wasm_bindgen(method, getter)]
    pub fn a(this: &JsMatrix) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn b(this: &JsMatrix) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn c(this: &JsMatrix) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn d(this: &JsMatrix) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn tx(this: &JsMatrix) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn ty(this: &JsMatrix) -> f64;
}
