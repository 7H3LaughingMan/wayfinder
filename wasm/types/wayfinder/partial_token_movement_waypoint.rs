use crate::types::{
    foundry::{TokenShapeType, documents::JsPartialTokenMovementWaypoint},
    wayfinder::TokenMovementWaypoint,
};

#[derive(Clone, Debug)]
pub struct PartialTokenMovementWaypoint {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub elevation: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub depth: Option<f64>,
    pub shape: Option<TokenShapeType>,
    pub level: Option<String>,
    pub action: Option<String>,
    pub snapped: Option<bool>,
    pub explicit: Option<bool>,
    pub checkpoint: Option<bool>,
}

impl PartialTokenMovementWaypoint {
    pub fn create_waypoint(&self, default: &TokenMovementWaypoint) -> TokenMovementWaypoint {
        TokenMovementWaypoint {
            x: if let Some(x) = self.x { x } else { default.x },
            y: if let Some(y) = self.y { y } else { default.y },
            elevation: if let Some(elevation) = self.elevation { elevation } else { default.elevation },
            width: if let Some(width) = self.width { width } else { default.width },
            height: if let Some(height) = self.height { height } else { default.height },
            depth: if let Some(depth) = self.depth { depth } else { default.depth },
            shape: if let Some(shape) = self.shape { shape } else { default.shape },
            level: if let Some(level) = &self.level { level.clone() } else { default.level.clone() },
            action: if let Some(action) = &self.action { action.clone() } else { default.action.clone() },
            snapped: if let Some(snapped) = self.snapped { snapped } else { default.snapped },
            explicit: if let Some(explicit) = self.explicit { explicit } else { default.explicit },
            checkpoint: if let Some(checkpoint) = self.checkpoint { checkpoint } else { default.checkpoint },
        }
    }
}

impl From<JsPartialTokenMovementWaypoint> for PartialTokenMovementWaypoint {
    fn from(value: JsPartialTokenMovementWaypoint) -> Self {
        PartialTokenMovementWaypoint {
            x: value.x(),
            y: value.y(),
            elevation: value.elevation(),
            width: value.width(),
            height: value.height(),
            depth: value.depth(),
            shape: value.shape(),
            level: value.level(),
            action: value.action(),
            snapped: value.snapped(),
            explicit: value.explicit(),
            checkpoint: value.checkpoint(),
        }
    }
}
