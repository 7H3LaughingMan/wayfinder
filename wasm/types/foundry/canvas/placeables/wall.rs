use crate::types::foundry::documents::JsWallDocument;
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(
        extends = Object,
        js_name = Wall,
        js_namespace = ["foundry", "canvas", "placeables"],
        typescript_type = "foundry.canvas.placeables.Wall"
    )]
    pub type JsWall;

    #[wasm_bindgen(method, getter)]
    pub fn document(this: &JsWall) -> JsWallDocument;
}
