use crate::types::foundry::TokenShapeType;
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(
        extends = Object,
        js_name = TokenDocument,
        js_namespace = ["foundry", "documents"],
        typescript_type = "foundry.documents.TokenDocument"
    )]
    pub type JsTokenDocument;

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &JsTokenDocument) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn name(this: &JsTokenDocument) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn x(this: &JsTokenDocument) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn y(this: &JsTokenDocument) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn elevation(this: &JsTokenDocument) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn width(this: &JsTokenDocument) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn height(this: &JsTokenDocument) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn depth(this: &JsTokenDocument) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn shape(this: &JsTokenDocument) -> TokenShapeType;

    #[wasm_bindgen(method, getter)]
    pub fn level(this: &JsTokenDocument) -> String;

    #[wasm_bindgen(method, getter = movementAction)]
    pub fn movement_action(this: &JsTokenDocument) -> String;
}
