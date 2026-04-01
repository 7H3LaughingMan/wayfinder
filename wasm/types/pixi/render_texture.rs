use crate::types::pixi::{JsFramebuffer, JsRectangle, MSAA_QUALITY};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(
        extends = Object,
        js_name = RenderTexture,
        js_namespace = PIXI,
        typescript_type = "PIXI.RenderTexture"
    )]
    pub type JsRenderTexture;

    #[wasm_bindgen(method, getter)]
    pub fn frame(this: &JsRenderTexture) -> JsRectangle;

    #[wasm_bindgen(method, getter)]
    pub fn framebuffer(this: &JsRenderTexture) -> JsFramebuffer;

    #[wasm_bindgen(method, getter)]
    pub fn height(this: &JsRenderTexture) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn multisample(this: &JsRenderTexture) -> MSAA_QUALITY;

    #[wasm_bindgen(method, getter)]
    pub fn resolution(this: &JsRenderTexture) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn rotate(this: &JsRenderTexture) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn width(this: &JsRenderTexture) -> f64;
}
