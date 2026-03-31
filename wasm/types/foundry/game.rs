use crate::types::foundry::helpers::JsClientSettings;
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(
        extends = Object,
        js_name = Game,
        js_namespace = "foundry",
        typescript_type = "foundry.Game"
    )]
    pub type JsGame;

    #[wasm_bindgen(method, getter)]
    pub fn settings(this: &JsGame) -> JsClientSettings;
}
