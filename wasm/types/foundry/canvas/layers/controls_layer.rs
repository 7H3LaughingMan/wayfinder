use crate::types::pixi::JsGraphics;
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(
        extends = Object,
        js_name = ControlsLayer,
        js_namespace = ["foundry", "canvas", "layers"],
        typescript_type = "foundry.canvas.layers.ControlsLayer"
    )]
    pub type JsControlsLayer;

    #[wasm_bindgen(method, getter)]
    pub fn debug(this: &JsControlsLayer) -> JsGraphics;
}
