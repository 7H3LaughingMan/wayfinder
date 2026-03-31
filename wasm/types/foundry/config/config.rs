use crate::types::foundry::config::JsTokenConfig;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen]
    pub type JsConfig;

    #[wasm_bindgen(method, getter = Token)]
    pub fn token(this: &JsConfig) -> JsTokenConfig;
}
