use crate::CLIPPER_SCALING_FACTOR;
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(extends = Object, js_name = IntPoint, js_namespace = ClipperLib)]
    pub type JsIntPoint;

    #[wasm_bindgen(constructor, js_class = IntPoint, js_namespace = ClipperLib)]
    pub fn new(x: f64, y: f64) -> JsIntPoint;

    #[wasm_bindgen(method, getter = X)]
    pub fn x(this: &JsIntPoint) -> f64;

    #[wasm_bindgen(method, getter = Y)]
    pub fn y(this: &JsIntPoint) -> f64;
}

impl From<geo::Coord> for JsIntPoint {
    fn from(geo::Coord { x, y }: geo::Coord) -> Self {
        JsIntPoint::new((x * CLIPPER_SCALING_FACTOR).round(), (y * CLIPPER_SCALING_FACTOR).round())
    }
}

impl From<JsIntPoint> for geo::Coord {
    fn from(value: JsIntPoint) -> Self {
        geo::Coord { x: value.x() / CLIPPER_SCALING_FACTOR, y: value.y() / CLIPPER_SCALING_FACTOR }
    }
}
