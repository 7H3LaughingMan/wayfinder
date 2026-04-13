use crate::types::{foundry::TokenShapeType, helpers::JsObject, wayfinder::PartialTokenMovementWaypoint};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(typescript_type = "Partial<TokenMovementWaypoint>")]
    pub type JsPartialTokenMovementWaypoint;

    #[wasm_bindgen(method, getter)]
    pub fn x(this: &JsPartialTokenMovementWaypoint) -> Option<f64>;

    #[wasm_bindgen(method, getter)]
    pub fn y(this: &JsPartialTokenMovementWaypoint) -> Option<f64>;

    #[wasm_bindgen(method, getter)]
    pub fn elevation(this: &JsPartialTokenMovementWaypoint) -> Option<f64>;

    #[wasm_bindgen(method, getter)]
    pub fn width(this: &JsPartialTokenMovementWaypoint) -> Option<f64>;

    #[wasm_bindgen(method, getter)]
    pub fn height(this: &JsPartialTokenMovementWaypoint) -> Option<f64>;

    #[wasm_bindgen(method, getter)]
    pub fn depth(this: &JsPartialTokenMovementWaypoint) -> Option<f64>;

    #[wasm_bindgen(method, getter)]
    pub fn shape(this: &JsPartialTokenMovementWaypoint) -> Option<TokenShapeType>;

    #[wasm_bindgen(method, getter)]
    pub fn level(this: &JsPartialTokenMovementWaypoint) -> Option<String>;

    #[wasm_bindgen(method, getter)]
    pub fn action(this: &JsPartialTokenMovementWaypoint) -> Option<String>;

    #[wasm_bindgen(method, getter)]
    pub fn snapped(this: &JsPartialTokenMovementWaypoint) -> Option<bool>;

    #[wasm_bindgen(method, getter)]
    pub fn explicit(this: &JsPartialTokenMovementWaypoint) -> Option<bool>;

    #[wasm_bindgen(method, getter)]
    pub fn checkpoint(this: &JsPartialTokenMovementWaypoint) -> Option<bool>;
}

impl From<PartialTokenMovementWaypoint> for JsPartialTokenMovementWaypoint {
    fn from(value: PartialTokenMovementWaypoint) -> Self {
        JsObject::new()
            .set("x", value.x)
            .set("y", value.y)
            .set("elevation", value.elevation)
            .set("width", value.width)
            .set("height", value.height)
            .set("depth", value.depth)
            .set("shape", value.shape)
            .set("level", value.level)
            .set("action", value.action)
            .set("snapped", value.snapped)
            .set("explicit", value.explicit)
            .set("checkpoint", value.checkpoint)
            .unchecked_into()
    }
}
