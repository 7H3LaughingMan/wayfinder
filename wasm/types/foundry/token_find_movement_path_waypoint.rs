use crate::types::{foundry::TokenShapeType, helpers::JsObject, wayfinder::TokenFindMovementPathWaypoint};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(typescript_type = "foundry.TokenFindMovementPathWaypoint")]
    pub type JsTokenFindMovementPathWaypoint;

    #[wasm_bindgen(method, getter)]
    pub fn x(this: &JsTokenFindMovementPathWaypoint) -> Option<f64>;

    #[wasm_bindgen(method, getter)]
    pub fn y(this: &JsTokenFindMovementPathWaypoint) -> Option<f64>;

    #[wasm_bindgen(method, getter)]
    pub fn elevation(this: &JsTokenFindMovementPathWaypoint) -> Option<f64>;

    #[wasm_bindgen(method, getter)]
    pub fn width(this: &JsTokenFindMovementPathWaypoint) -> Option<f64>;

    #[wasm_bindgen(method, getter)]
    pub fn height(this: &JsTokenFindMovementPathWaypoint) -> Option<f64>;

    #[wasm_bindgen(method, getter)]
    pub fn shape(this: &JsTokenFindMovementPathWaypoint) -> Option<TokenShapeType>;

    #[wasm_bindgen(method, getter)]
    pub fn action(this: &JsTokenFindMovementPathWaypoint) -> Option<String>;

    #[wasm_bindgen(method, getter)]
    pub fn snapped(this: &JsTokenFindMovementPathWaypoint) -> Option<bool>;

    #[wasm_bindgen(method, getter)]
    pub fn explicit(this: &JsTokenFindMovementPathWaypoint) -> Option<bool>;

    #[wasm_bindgen(method, getter)]
    pub fn checkpoint(this: &JsTokenFindMovementPathWaypoint) -> Option<bool>;
}

impl From<TokenFindMovementPathWaypoint> for JsTokenFindMovementPathWaypoint {
    fn from(value: TokenFindMovementPathWaypoint) -> Self {
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
