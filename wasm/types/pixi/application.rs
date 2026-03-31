use crate::types::pixi::JsRenderer;
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = Object,
        js_name = Application,
        js_namespace = PIXI,
        typescript_type = "PIXI.Application"
    )]
    pub type JsApplication;

    #[wasm_bindgen(method, getter)]
    pub fn renderer(this: &JsApplication) -> JsRenderer;
}
