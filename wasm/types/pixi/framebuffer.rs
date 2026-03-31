use crate::types::pixi::{JsGlFramebuffers, MSAA_QUALITY};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = Object,
        js_name = Framebuffer,
        js_namespace = PIXI,
        typescript_type = "PIXI.Framebuffer"
    )]
    pub type JsFramebuffer;

    #[wasm_bindgen(constructor, js_class = Framebuffer, js_namespace = PIXI)]
    pub fn new(width: f64, height: f64) -> JsFramebuffer;

    #[wasm_bindgen(method, getter)]
    pub fn depth(this: &JsFramebuffer) -> bool;

    #[wasm_bindgen(method, getter = dirtyFormat)]
    pub fn dirty_format(this: &JsFramebuffer) -> f64;

    #[wasm_bindgen(method, getter = dirtyId)]
    pub fn dirty_id(this: &JsFramebuffer) -> f64;

    #[wasm_bindgen(method, getter = dirtySize)]
    pub fn dirty_size(this: &JsFramebuffer) -> f64;

    #[wasm_bindgen(method, getter = glFramebuffers)]
    pub fn gl_framebuffers(this: &JsFramebuffer) -> JsGlFramebuffers;

    #[wasm_bindgen(method, getter)]
    pub fn height(this: &JsFramebuffer) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn multisample(this: &JsFramebuffer) -> MSAA_QUALITY;

    #[wasm_bindgen(method, getter)]
    pub fn stencil(this: &JsFramebuffer) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn width(this: &JsFramebuffer) -> f64;
}
