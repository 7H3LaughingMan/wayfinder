use wasm_bindgen::JsValue;

use crate::{
    enums::{WallDirection, WallDoorState, WallDoorType, WallMovementType, WallSenseType},
    traits::{JsDeserialize, JsHelper},
};

#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct Wall {
    pub c: [f32; 4],
    pub light: WallSenseType,
    pub r#move: WallMovementType,
    pub sight: WallSenseType,
    pub sound: WallSenseType,
    pub dir: WallDirection,
    pub door: WallDoorType,
    pub ds: WallDoorState,
}

impl Wall {
    pub fn blocks_movement(&self) -> bool {
        if self.door == WallDoorType::None && self.r#move == WallMovementType::Normal {
            return true;
        }

        if self.door != WallDoorType::None && self.ds != WallDoorState::Open {
            return true;
        }

        false
    }
}

impl JsDeserialize for Wall {
    fn from_value(value: JsValue) -> Self {
        let c: Vec<f32> = {
            let value = value.get("c");
            let mut vector = Vec::new();
            let iterator = js_sys::try_iter(&value).unwrap().unwrap();

            for x in iterator {
                vector.push(JsDeserialize::from_value(x.unwrap()));
            }

            vector
        };

        Wall {
            c: [c[0], c[1], c[2], c[3]],
            light: JsDeserialize::from_value(value.get("light")),
            r#move: JsDeserialize::from_value(value.get("move")),
            sight: JsDeserialize::from_value(value.get("sight")),
            sound: JsDeserialize::from_value(value.get("sound")),
            dir: JsDeserialize::from_value(value.get("dir")),
            door: JsDeserialize::from_value(value.get("door")),
            ds: JsDeserialize::from_value(value.get("ds")),
        }
    }
}

impl From<Wall> for rapier2d::prelude::Collider {
    fn from(value: Wall) -> Self {
        let a = rapier2d::na::Point2::new(value.c[0], value.c[1]);
        let b = rapier2d::na::Point2::new(value.c[2], value.c[3]);

        rapier2d::prelude::ColliderBuilder::polyline(vec![a, b], None).build()
    }
}
