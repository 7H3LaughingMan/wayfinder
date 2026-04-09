use crate::types::{
    foundry::{TokenShapeType, documents::JsTokenMovementWaypoint},
    wayfinder::ElevatedPoint,
};

#[derive(Clone, Debug)]
pub struct TokenMovementWaypoint {
    pub x: f64,
    pub y: f64,
    pub elevation: f64,
    pub width: f64,
    pub height: f64,
    pub shape: TokenShapeType,
    pub action: String,
    pub snapped: bool,
    pub explicit: bool,
    pub checkpoint: bool,
}

impl TokenMovementWaypoint {
    pub fn create_elevated_point(&self) -> ElevatedPoint {
        ElevatedPoint { x: self.x, y: self.y, elevation: self.elevation }
    }

    pub fn from_elevated_point(
        &self,
        ElevatedPoint { x, y, elevation }: ElevatedPoint,
        snapped: bool,
        explicit: bool,
        checkpoint: bool,
    ) -> TokenMovementWaypoint {
        TokenMovementWaypoint {
            x,
            y,
            elevation,
            width: self.width,
            height: self.height,
            shape: self.shape,
            action: self.action.clone(),
            snapped,
            explicit,
            checkpoint,
        }
    }
}

impl From<JsTokenMovementWaypoint> for TokenMovementWaypoint {
    fn from(value: JsTokenMovementWaypoint) -> Self {
        TokenMovementWaypoint {
            x: value.x(),
            y: value.y(),
            elevation: value.elevation(),
            width: value.width(),
            height: value.height(),
            shape: value.shape(),
            action: value.action(),
            snapped: value.snapped(),
            explicit: value.explicit(),
            checkpoint: value.checkpoint(),
        }
    }
}
