use crate::types::{
    helpers::JsObject,
    wayfinder::{GridOffset2D, GridOffset3D},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(typescript_type = "foundry.grid.GridOffset2D")]
    pub type JsGridOffset2D;

    #[wasm_bindgen(method, getter)]
    pub fn i(this: &JsGridOffset2D) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn j(this: &JsGridOffset2D) -> f64;
}

impl From<GridOffset2D> for JsGridOffset2D {
    fn from(GridOffset2D { i, j }: GridOffset2D) -> Self {
        JsObject::new().set("i", i).set("j", j).unchecked_into()
    }
}

impl From<GridOffset3D> for JsGridOffset2D {
    fn from(GridOffset3D { i, j, k: _ }: GridOffset3D) -> Self {
        JsObject::new().set("i", i).set("j", j).unchecked_into()
    }
}
