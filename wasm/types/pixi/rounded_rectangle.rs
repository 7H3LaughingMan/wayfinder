use crate::types::shapes::RoundedRectangle;
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = Object,
        js_name = RoundedRectangle,
        js_namespace = PIXI,
        typescript_type = "PIXI.RoundedRectangle"
    )]
    pub type JsRoundedRectangle;

    #[wasm_bindgen(constructor, js_class = RoundedRectangle, js_namespace = PIXI)]
    pub fn new(x: f64, y: f64, width: f64, height: f64, radius: f64) -> JsRoundedRectangle;

    #[wasm_bindgen(method, getter)]
    pub fn height(this: &JsRoundedRectangle) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn radius(this: &JsRoundedRectangle) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn width(this: &JsRoundedRectangle) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn x(this: &JsRoundedRectangle) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn y(this: &JsRoundedRectangle) -> f64;

    #[wasm_bindgen(method)]
    pub fn clone(this: &JsRoundedRectangle) -> JsRoundedRectangle;

    #[wasm_bindgen(method)]
    pub fn contains(this: &JsRoundedRectangle, x: f64, y: f64) -> bool;
}

impl From<RoundedRectangle> for JsRoundedRectangle {
    fn from(value: RoundedRectangle) -> Self {
        JsRoundedRectangle::new(value.x, value.y, value.width, value.height, value.radius)
    }
}
