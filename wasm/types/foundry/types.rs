use crate::types::{
    helpers::JsObject,
    wayfinder::{ElevatedPoint, Point},
};
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

impl From<Point> for JsPoint {
    fn from(Point { x, y }: Point) -> Self {
        JsPoint::new(x, y)
    }
}

impl From<ElevatedPoint> for JsPoint {
    fn from(ElevatedPoint { x, y, elevation: _ }: ElevatedPoint) -> Self {
        JsPoint::new(x, y)
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

impl From<Point> for JsElevatedPoint {
    fn from(Point { x, y }: Point) -> Self {
        JsElevatedPoint::new(x, y, 0)
    }
}

impl From<ElevatedPoint> for JsElevatedPoint {
    fn from(ElevatedPoint { x, y, elevation }: ElevatedPoint) -> Self {
        JsElevatedPoint::new(x, y, elevation)
    }
}
