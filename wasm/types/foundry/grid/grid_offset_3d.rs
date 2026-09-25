use crate::types::{
    helpers::JsObject,
    wayfinder::{GridOffset2D, GridOffset3D},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(typescript_type = "foundry.grid.GridOffset3D")]
    pub type JsGridOffset3D;

    #[wasm_bindgen(method, getter)]
    pub fn i(this: &JsGridOffset3D) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn j(this: &JsGridOffset3D) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn k(this: &JsGridOffset3D) -> f64;
}

impl From<GridOffset2D> for JsGridOffset3D {
    fn from(GridOffset2D { i, j }: GridOffset2D) -> Self {
        JsObject::new().set("i", i).set("j", j).set("k", 0).unchecked_into()
    }
}

impl From<GridOffset3D> for JsGridOffset3D {
    fn from(GridOffset3D { i, j, k }: GridOffset3D) -> Self {
        JsObject::new().set("i", i).set("j", j).set("k", k).unchecked_into()
    }
}
