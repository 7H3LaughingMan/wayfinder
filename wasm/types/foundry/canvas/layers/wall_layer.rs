use crate::types::foundry::canvas::placeables::JsWall;
use js_sys::{Array, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(
        extends = Object,
        js_name = WallsLayer,
        js_namespace = ["foundry", "canvas", "layers"],
        typescript_type = "foundry.canvas.layers.WallsLayer"
    )]
    pub type JsWallsLayer;

    #[wasm_bindgen(method, getter)]
    pub fn placeables(this: &JsWallsLayer) -> Array<JsWall>;
}
