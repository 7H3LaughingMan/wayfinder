use crate::types::{
    foundry::{TokenShapeType, documents::JsTokenDocument},
    wayfinder::TokenMovementWaypoint,
};

#[derive(Clone, Debug)]
pub struct TokenDocument {
    pub x: f64,
    pub y: f64,
    pub elevation: f64,
    pub width: f64,
    pub height: f64,
    pub depth: f64,
    pub shape: TokenShapeType,
    pub level: String,
    pub movement_action: String,
}

impl TokenDocument {
    pub fn create_waypoint(&self) -> TokenMovementWaypoint {
        TokenMovementWaypoint {
            x: self.x,
            y: self.y,
            elevation: self.elevation,
            width: self.width,
            height: self.height,
            depth: self.depth,
            shape: self.shape,
            level: self.level.clone(),
            action: self.movement_action.clone(),
            snapped: false,
            explicit: false,
            checkpoint: false,
        }
    }
}

impl From<JsTokenDocument> for TokenDocument {
    fn from(value: JsTokenDocument) -> Self {
        TokenDocument {
            x: value.x(),
            y: value.y(),
            elevation: value.elevation(),
            width: value.width(),
            height: value.height(),
            depth: value.depth(),
            shape: value.shape(),
            level: value.level(),
            movement_action: value.movement_action(),
        }
    }
}
