use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(
        extends = Object,
        js_name = SpriteMesh,
        js_namespace = ["foundry", "canvas", "controls"],
        typescript_type = "foundry.canvas.controls.SpriteMesh"
    )]
    pub type JsSpriteMesh;

    #[wasm_bindgen(method, getter)]
    pub fn x(this: &JsSpriteMesh) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn y(this: &JsSpriteMesh) -> f64;
}
