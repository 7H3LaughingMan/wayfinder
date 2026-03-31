use crate::types::{pixi::JsRectangle, shapes::Circle};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = Object,
        js_name = Circle,
        js_namespace = PIXI,
        typescript_type = "PIXI.Circle"
    )]
    pub type JsCircle;

    #[wasm_bindgen(constructor, js_class = Circle, js_namespace = PIXI)]
    pub fn new(x: f64, y: f64, radius: f64) -> JsCircle;

    #[wasm_bindgen(method, getter)]
    pub fn radius(this: &JsCircle) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn x(this: &JsCircle) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn y(this: &JsCircle) -> f64;

    #[wasm_bindgen(method)]
    pub fn clone(this: &JsCircle) -> JsCircle;

    #[wasm_bindgen(method)]
    pub fn contains(this: &JsCircle, x: f64, y: f64) -> bool;

    #[wasm_bindgen(method)]
    pub fn get_bounds(this: &JsCircle) -> JsRectangle;
}

impl From<Circle> for JsCircle {
    fn from(value: Circle) -> Self {
        JsCircle::new(value.x, value.y, value.radius)
    }
}
