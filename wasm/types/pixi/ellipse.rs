use crate::types::{pixi::JsRectangle, shapes::Ellipse};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = Object,
        js_name = Ellipse,
        js_namespace = PIXI,
        typescript_type = "PIXI.Ellipse"
    )]
    pub type JsEllipse;

    #[wasm_bindgen(constructor, js_class = Ellipse, js_namespace = PIXI)]
    pub fn new(x: f64, y: f64, half_width: f64, half_height: f64) -> JsEllipse;

    #[wasm_bindgen(method, getter)]
    pub fn height(this: &JsEllipse) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn width(this: &JsEllipse) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn x(this: &JsEllipse) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn y(this: &JsEllipse) -> f64;

    #[wasm_bindgen(method)]
    pub fn clone(this: &JsEllipse) -> JsEllipse;

    #[wasm_bindgen(method)]
    pub fn contains(this: &JsEllipse, x: f64, y: f64) -> bool;

    #[wasm_bindgen(method)]
    pub fn get_bounds(this: &JsEllipse) -> JsRectangle;
}

impl From<Ellipse> for JsEllipse {
    fn from(value: Ellipse) -> Self {
        JsEllipse::new(value.x, value.y, value.half_width, value.half_height)
    }
}
