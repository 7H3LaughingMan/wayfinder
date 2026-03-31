use crate::types::pixi::{JsFramebuffer, MSAA_QUALITY};
use js_sys::{JsOption, Object};
use wasm_bindgen::prelude::*;
use web_sys::{WebGlFramebuffer, WebGlRenderbuffer, WebGlTexture};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = Object,
        js_name = GLFramebuffer,
        js_namespace = PIXI,
        typescript_type = "PIXI.GLFramebuffer"
    )]
    pub type JsGlFramebuffer;

    #[wasm_bindgen(constructor, js_class = GLFramebuffer, js_namespace = PIXI)]
    pub fn new(framebuffer: WebGlTexture) -> JsGlFramebuffer;

    #[wasm_bindgen(method, getter)]
    pub fn framebuffer(this: &JsGlFramebuffer) -> WebGlFramebuffer;

    #[wasm_bindgen(method, getter)]
    pub fn stencil(this: &JsGlFramebuffer) -> JsOption<WebGlRenderbuffer>;

    #[wasm_bindgen(method, getter = dirtyId)]
    pub fn dirty_id(this: &JsGlFramebuffer) -> f64;

    #[wasm_bindgen(method, getter = dirtyFormat)]
    pub fn dirty_format(this: &JsGlFramebuffer) -> f64;

    #[wasm_bindgen(method, getter = dirtySize)]
    pub fn dirty_size(this: &JsGlFramebuffer) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn multisample(this: &JsGlFramebuffer) -> MSAA_QUALITY;

    #[wasm_bindgen(method, getter = msaaBuffer)]
    pub fn msaa_buffer(this: &JsGlFramebuffer) -> JsOption<WebGlRenderbuffer>;

    #[wasm_bindgen(method, getter = blitFramebuffer)]
    pub fn blit_framebuffer(this: &JsGlFramebuffer) -> JsOption<JsFramebuffer>;

    #[wasm_bindgen(method, getter = mipLevel)]
    pub fn mip_levelt(this: &JsGlFramebuffer) -> f64;
}
