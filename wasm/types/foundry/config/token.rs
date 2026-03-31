use wasm_bindgen::prelude::*;

use crate::types::helpers::JsObject;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen]
    pub type JsTokenConfig;

    #[wasm_bindgen(method, getter)]
    pub fn movement(this: &JsTokenConfig) -> JsTokenConfigMovement;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen]
    pub type JsTokenConfigMovement;

    #[wasm_bindgen(method, getter)]
    pub fn actions(this: &JsTokenConfigMovement) -> JsObject;
}
