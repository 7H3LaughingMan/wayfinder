use js_sys::Object;
use wasm_bindgen::prelude::*;
use web_sys::WebGlTexture;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = Object,
        js_name = GLTexture,
        js_namespace = PIXI,
        typescript_type = "PIXI.GLTexture"
    )]
    pub type JsGlTexture;

    #[wasm_bindgen(constructor, js_class = GLTexture, js_namespace = PIXI)]
    pub fn new(texture: WebGlTexture) -> JsGlTexture;

    #[wasm_bindgen(method, getter = dirtyId)]
    pub fn dirty_id(this: &JsGlTexture) -> f64;

    #[wasm_bindgen(method, getter = dirtyStyleId)]
    pub fn dirty_style_id(this: &JsGlTexture) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn height(this: &JsGlTexture) -> i32;

    #[wasm_bindgen(method, getter = internalFormat)]
    pub fn internal_format(this: &JsGlTexture) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn mipmap(this: &JsGlTexture) -> f64;

    #[wasm_bindgen(method, getter = samplerType)]
    pub fn sampler_type(this: &JsGlTexture) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn texture(this: &JsGlTexture) -> WebGlTexture;

    #[wasm_bindgen(method, getter = type)]
    pub fn r#type(this: &JsGlTexture) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn width(this: &JsGlTexture) -> i32;

    #[wasm_bindgen(method, getter = wrapMode)]
    pub fn wrap_mode(this: &JsGlTexture) -> f64;
}
