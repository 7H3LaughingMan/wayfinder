use crate::types::pixi::JsGlFramebuffer;
use js_sys::{JsOption, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = Object,
        typescript_type = "{[key: string]: PIXI.GLFramebuffer}"
    )]
    pub type JsGlFramebuffers;

    #[wasm_bindgen(method, indexing_getter)]
    pub fn get(this: &JsGlFramebuffers, prop: u32) -> JsOption<JsGlFramebuffer>;
}
