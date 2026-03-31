use crate::types::foundry::utils::JsColor;
use geo::{Polygon, Triangle};
use itertools::Itertools;
use js_sys::{Float32Array, Object, Uint16Array, Uint32Array};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(
        extends = Object,
        js_name = RegionDocument,
        js_namespace = ["foundry", "documents"],
        typescript_type = "foundry.documents.RegionDocument"
    )]
    pub type JsRegionDocument;

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &JsRegionDocument) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn color(this: &JsRegionDocument) -> JsColor;

    #[wasm_bindgen(method, getter)]
    pub fn elevation(this: &JsRegionDocument) -> JsElevationRange;

    #[wasm_bindgen(method, getter)]
    pub fn triangulation(this: &JsRegionDocument) -> JsRegionTriangulation;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen]
    pub type JsElevationRange;

    #[wasm_bindgen(method, getter)]
    pub fn bottom(this: &JsElevationRange) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn top(this: &JsElevationRange) -> f64;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen]
    pub type JsRegionTriangulation;

    #[wasm_bindgen(method, getter)]
    pub fn vertices(this: &JsRegionTriangulation) -> Float32Array;

    #[wasm_bindgen(method, getter)]
    pub fn indices(this: &JsRegionTriangulation) -> JsValue;
}

impl JsRegionTriangulation {
    pub fn to_polygons(&self) -> Vec<Polygon> {
        let vertices = self
            .vertices()
            .to_vec()
            .into_iter()
            .tuples::<(f32, f32)>()
            .map(|(a, b)| (a as f64, b as f64))
            .collect_vec();

        if let Some(uint16_array) = self.indices().dyn_ref::<Uint16Array>() {
            uint16_array
                .to_vec()
                .into_iter()
                .tuples::<(u16, u16, u16)>()
                .map(|(a, b, c)| [vertices[a as usize], vertices[b as usize], vertices[c as usize]])
                .map(Triangle::from)
                .map(Polygon::from)
                .collect()
        } else if let Some(uint32_array) = self.indices().dyn_ref::<Uint32Array>() {
            uint32_array
                .to_vec()
                .into_iter()
                .tuples::<(u32, u32, u32)>()
                .map(|(a, b, c)| [vertices[a as usize], vertices[b as usize], vertices[c as usize]])
                .map(Triangle::from)
                .map(Polygon::from)
                .collect()
        } else {
            Vec::new()
        }
    }
}
