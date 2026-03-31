use crate::types::foundry::documents::JsRegionDocument;
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(
        extends = Object,
        js_name = Region,
        js_namespace = ["foundry", "canvas", "placeables"],
        typescript_type = "foundry.canvas.placeables.Region"
    )]
    pub type JsRegion;

    #[wasm_bindgen(method, getter)]
    pub fn document(this: &JsRegion) -> JsRegionDocument;
}
