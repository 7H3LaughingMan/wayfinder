use crate::types::foundry::canvas::placeables::JsRegion;
use js_sys::{Array, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(
        extends = Object,
        js_name = RegionLayer,
        js_namespace = ["foundry", "canvas", "layers"],
        typescript_type = "foundry.canvas.layers.RegionLayer"
    )]
    pub type JsRegionLayer;

    #[wasm_bindgen(method, getter)]
    pub fn placeables(this: &JsRegionLayer) -> Array<JsRegion>;
}
