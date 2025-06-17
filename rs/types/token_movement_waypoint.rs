use crate::{
    enums::TokenShapeType,
    traits::{JsHelper, JsSerialize},
    types::ElevatedPoint,
};

#[derive(Clone)]
#[derive(Debug)]
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

impl JsSerialize for &TokenMovementWaypoint {
    fn to_value(value: Self) -> wasm_bindgen::JsValue {
        let object = js_sys::Object::new();

        object.set("x", JsSerialize::to_value(value.x));
        object.set("y", JsSerialize::to_value(value.y));
        object.set("elevation", JsSerialize::to_value(value.elevation));
        object.set("width", JsSerialize::to_value(value.width));
        object.set("height", JsSerialize::to_value(value.height));
        object.set("shape", JsSerialize::to_value(value.shape));
        object.set("action", JsSerialize::to_value(value.action.clone()));
        object.set("snapped", JsSerialize::to_value(value.snapped));
        object.set("explicit", JsSerialize::to_value(value.explicit));
        object.set("checkpoint", JsSerialize::to_value(value.checkpoint));

        return object.into();
    }
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
            snapped: snapped,
            explicit: explicit,
            checkpoint: checkpoint,
        }
    }
}
