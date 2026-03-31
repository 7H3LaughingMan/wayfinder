use crate::types::helpers::JsObject;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type JsPoint;

    #[wasm_bindgen(method, getter)]
    pub fn x(this: &JsPoint) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn y(this: &JsPoint) -> f64;
}

impl JsPoint {
    pub fn new(x: impl Into<f64>, y: impl Into<f64>) -> Self {
        JsObject::new().set("x", x.into()).set("y", y.into()).unchecked_into()
    }
}

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type JsElevatedPoint;

    #[wasm_bindgen(method, getter)]
    pub fn x(this: &JsElevatedPoint) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn y(this: &JsElevatedPoint) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn elevation(this: &JsElevatedPoint) -> f64;
}

impl JsElevatedPoint {
    pub fn new(x: impl Into<f64>, y: impl Into<f64>, elevation: impl Into<f64>) -> Self {
        JsObject::new().set("x", x.into()).set("y", y.into()).set("elevation", elevation.into()).unchecked_into()
    }
}
