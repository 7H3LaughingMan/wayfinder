use crate::traits::{JsDeserialize, JsHelper};
use crate::types::Point;
use wasm_bindgen::JsValue;

#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    pub fn contains(&self, Point { x, y }: Point) -> bool {
        if self.width <= 0.0 || self.height <= 0.0 {
            return false;
        }

        if x >= self.x && x < self.x + self.width {
            if y >= self.y && y < self.y + self.height {
                return true;
            }
        }

        return false;
    }

    pub fn contains_rectangle(&self, other: Rectangle) -> bool {
        if other.width <= 0.0 || other.height <= 0.0 {
            return other.x > self.x && other.y > self.y && other.right() < self.right() && other.bottom() < self.bottom();
        }

        return other.x >= self.x && other.y >= self.y && other.right() <= self.right() && other.bottom() <= self.bottom();
    }

    pub fn left(&self) -> f32 {
        self.x
    }

    pub fn right(&self) -> f32 {
        self.x + self.width
    }

    pub fn top(&self) -> f32 {
        self.y
    }

    pub fn bottom(&self) -> f32 {
        self.y + self.height
    }
}

impl JsDeserialize for Rectangle {
    fn from_value(value: JsValue) -> Self {
        Rectangle {
            x: JsDeserialize::from_value(value.get("x")),
            y: JsDeserialize::from_value(value.get("y")),
            width: JsDeserialize::from_value(value.get("width")),
            height: JsDeserialize::from_value(value.get("height")),
        }
    }
}
