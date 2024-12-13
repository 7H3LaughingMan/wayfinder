use crate::{
    log,
    traits::{JsDeserialize, JsHelper},
    types::{circle, Point, Rectangle},
};

#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}

impl JsDeserialize for Circle {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        Self { x: data.get_value("x"), y: data.get_value("y"), radius: data.get_value("radius") }
    }
}
