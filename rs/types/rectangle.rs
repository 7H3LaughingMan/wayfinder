use crate::{
    traits::{JsDeserialize, JsHelper},
    types::Point,
};
use wasm_bindgen::JsValue;

#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self { x, y, width, height }
    }

    pub fn contains(&self, x: f64, y: f64) -> bool {
        if self.width <= 0.0 || self.height <= 0.0 {
            return false;
        }

        if x >= self.left() && x < self.right() {
            if y >= self.top() && y < self.bottom() {
                return true;
            }
        }

        false
    }

    pub fn contains_point(&self, Point { x, y }: Point) -> bool {
        self.contains(x, y)
    }

    pub fn normalize(&self) -> Rectangle {
        let Rectangle { mut x, mut y, mut width, mut height } = *self;

        if width < 0.0 {
            x += width;
            width = width.abs();
        }

        if height < 0.0 {
            y += height;
            height = height.abs();
        }

        return Rectangle { x, y, width, height };
    }

    pub fn overlaps(&self, other: &Rectangle) -> bool {
        return other.right() >= self.left()
            && other.left() <= self.right()
            && other.bottom() >= self.top()
            && other.top() <= self.bottom();
    }

    pub fn left(&self) -> f64 {
        self.x
    }

    pub fn right(&self) -> f64 {
        self.x + self.width
    }

    pub fn top(&self) -> f64 {
        self.y
    }

    pub fn bottom(&self) -> f64 {
        self.y + self.height
    }
}

impl JsDeserialize for Rectangle {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        Rectangle {
            x: data.get_value("x"),
            y: data.get_value("y"),
            width: data.get_value("width"),
            height: data.get_value("height"),
        }
    }
}
