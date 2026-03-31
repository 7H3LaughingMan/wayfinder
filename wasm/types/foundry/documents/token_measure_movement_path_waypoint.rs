use crate::types::{TokenMovementWaypoint, foundry::TokenShapeType, helpers::JsObject};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(typescript_type = "foundry.documents.TokenMeasureMovementPathWaypoint")]
    pub type JsTokenMeasureMovementPathWaypoint;

    #[wasm_bindgen(method, getter)]
    pub fn x(this: &JsTokenMeasureMovementPathWaypoint) -> Option<f64>;

    #[wasm_bindgen(method, getter)]
    pub fn y(this: &JsTokenMeasureMovementPathWaypoint) -> Option<f64>;

    #[wasm_bindgen(method, getter)]
    pub fn elevation(this: &JsTokenMeasureMovementPathWaypoint) -> Option<f64>;

    #[wasm_bindgen(method, getter)]
    pub fn width(this: &JsTokenMeasureMovementPathWaypoint) -> Option<f64>;

    #[wasm_bindgen(method, getter)]
    pub fn height(this: &JsTokenMeasureMovementPathWaypoint) -> Option<f64>;

    #[wasm_bindgen(method, getter)]
    pub fn shape(this: &JsTokenMeasureMovementPathWaypoint) -> Option<TokenShapeType>;

    #[wasm_bindgen(method, getter)]
    pub fn action(this: &JsTokenMeasureMovementPathWaypoint) -> Option<String>;
}

impl From<TokenMovementWaypoint> for JsTokenMeasureMovementPathWaypoint {
    fn from(value: TokenMovementWaypoint) -> Self {
        JsObject::new()
            .set("x", value.x)
            .set("y", value.y)
            .set("elevation", value.elevation)
            .set("width", value.width)
            .set("height", value.height)
            .set("shape", value.shape)
            .set("action", value.action)
            .unchecked_into()
    }
}
