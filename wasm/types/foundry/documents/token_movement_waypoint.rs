use crate::types::{foundry::TokenShapeType, helpers::JsObject, wayfinder::TokenMovementWaypoint};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(typescript_type = "foundry.documents.TokenMovementWaypoint")]
    pub type JsTokenMovementWaypoint;

    #[wasm_bindgen(method, getter)]
    pub fn x(this: &JsTokenMovementWaypoint) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn y(this: &JsTokenMovementWaypoint) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn elevation(this: &JsTokenMovementWaypoint) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn width(this: &JsTokenMovementWaypoint) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn height(this: &JsTokenMovementWaypoint) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn shape(this: &JsTokenMovementWaypoint) -> TokenShapeType;

    #[wasm_bindgen(method, getter)]
    pub fn action(this: &JsTokenMovementWaypoint) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn snapped(this: &JsTokenMovementWaypoint) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn explicit(this: &JsTokenMovementWaypoint) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn checkpoint(this: &JsTokenMovementWaypoint) -> bool;
}

impl From<TokenMovementWaypoint> for JsTokenMovementWaypoint {
    fn from(value: TokenMovementWaypoint) -> Self {
        JsObject::new()
            .set("x", value.x)
            .set("y", value.y)
            .set("elevation", value.elevation)
            .set("width", value.width)
            .set("height", value.height)
            .set("shape", value.shape)
            .set("action", value.action)
            .set("snapped", value.snapped)
            .set("explicit", value.explicit)
            .set("checkpoint", value.checkpoint)
            .unchecked_into()
    }
}
