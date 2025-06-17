use crate::{
    enums::TokenShapeType,
    traits::{JsDeserialize, JsHelper, JsSerialize},
    types::TokenMovementWaypoint,
};

#[derive(Clone)]
#[derive(Debug)]
pub struct TokenDocument {
    pub x: f64,
    pub y: f64,
    pub elevation: f64,
    pub width: f64,
    pub height: f64,
    pub shape: TokenShapeType,
    pub movement_action: String,
}

impl JsDeserialize for TokenDocument {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        TokenDocument {
            x: data.get_value("x"),
            y: data.get_value("y"),
            elevation: data.get_value("elevation"),
            width: data.get_value("width"),
            height: data.get_value("height"),
            shape: data.get_value("shape"),
            movement_action: data.get_value("movementAction"),
        }
    }
}

impl TokenDocument {
    pub fn create_waypoint(&self) -> TokenMovementWaypoint {
        TokenMovementWaypoint {
            x: self.x,
            y: self.y,
            elevation: self.elevation,
            width: self.width,
            height: self.height,
            shape: self.shape,
            action: self.movement_action.clone(),
            snapped: false,
            explicit: false,
            checkpoint: false,
        }
    }
}
