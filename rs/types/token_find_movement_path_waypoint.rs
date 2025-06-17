use crate::{
    enums::TokenShapeType,
    traits::{JsDeserialize, JsHelper},
    types::TokenMovementWaypoint,
};

#[derive(Clone)]
#[derive(Debug)]
pub struct TokenFindMovementPathWaypoint {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub elevation: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub shape: Option<TokenShapeType>,
    pub action: Option<String>,
    pub snapped: Option<bool>,
    pub explicit: Option<bool>,
    pub checkpoint: Option<bool>,
}

impl JsDeserialize for TokenFindMovementPathWaypoint {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        TokenFindMovementPathWaypoint {
            x: data.get_value("x"),
            y: data.get_value("y"),
            elevation: data.get_value("elevation"),
            width: data.get_value("width"),
            height: data.get_value("height"),
            shape: data.get_value("shape"),
            action: data.get_value("action"),
            snapped: data.get_value("snapped"),
            explicit: data.get_value("explicit"),
            checkpoint: data.get_value("checkpoint"),
        }
    }
}

impl TokenFindMovementPathWaypoint {
    pub fn create_waypoint(&self, default: &TokenMovementWaypoint) -> TokenMovementWaypoint {
        TokenMovementWaypoint {
            x: if let Some(x) = self.x { x } else { default.x },
            y: if let Some(y) = self.y { y } else { default.y },
            elevation: if let Some(elevation) = self.elevation { elevation } else { default.elevation },
            width: if let Some(width) = self.width { width } else { default.width },
            height: if let Some(height) = self.height { height } else { default.height },
            shape: if let Some(shape) = self.shape { shape } else { default.shape },
            action: if let Some(action) = &self.action { action.clone() } else { default.action.clone() },
            snapped: if let Some(snapped) = self.snapped { snapped } else { default.snapped },
            explicit: if let Some(explicit) = self.explicit { explicit } else { default.explicit },
            checkpoint: if let Some(checkpoint) = self.checkpoint { checkpoint } else { default.checkpoint },
        }
    }
}
