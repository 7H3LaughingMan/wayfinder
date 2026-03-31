use crate::types::{
    TokenMovementWaypoint,
    foundry::{TokenShapeType, documents::JsTokenDocument},
};

#[derive(Clone, Debug)]
pub struct TokenDocument {
    pub x: f64,
    pub y: f64,
    pub elevation: f64,
    pub width: f64,
    pub height: f64,
    pub shape: TokenShapeType,
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
            shape: self.shape,
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
            shape: value.shape(),
            movement_action: value.movement_action(),
        }
    }
}
