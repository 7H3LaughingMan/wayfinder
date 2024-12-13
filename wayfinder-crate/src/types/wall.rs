use crate::{
    enums::{WallDirection, WallDoorState, WallDoorType, WallMovementType, WallSenseType},
    traits::{JsDeserialize, JsHelper},
    types::Point,
};

use super::Rectangle;

#[derive(Clone)]
#[derive(Debug)]
pub struct Wall {
    pub id: String,
    pub c: [f64; 4],
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

    pub fn get_a(&self) -> Point {
        Point::new(self.c[0], self.c[1])
    }

    pub fn get_b(&self) -> Point {
        Point::new(self.c[2], self.c[3])
    }

    pub fn get_bounds(&self) -> Rectangle {
        let Point { x: x0, y: y0 } = self.get_a();
        let Point { x: x1, y: y1 } = self.get_b();

        return Rectangle::new(x0, y0, x1 - x0, y1 - y0).normalize();
    }
}

impl JsDeserialize for Wall {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        Wall {
            id: data.get_value("_id"),
            c: data.get_value("c"),
            light: data.get_value("light"),
            r#move: data.get_value("move"),
            sight: data.get_value("sight"),
            sound: data.get_value("sound"),
            dir: data.get_value("dir"),
            door: data.get_value("door"),
            ds: data.get_value("ds"),
        }
    }
}
