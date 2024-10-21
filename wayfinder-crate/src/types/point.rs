use crate::traits::{JsDeserialize, JsHelper, JsSerialize};
use wasm_bindgen::prelude::*;

#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn rotate(self, origin: Point, rotation: f32) -> Point {
        let s = rotation.to_radians().sin();
        let c = rotation.to_radians().cos();

        let offset = self - origin;
        let rotated = Point { x: (offset.x * c) - (offset.y * s), y: (offset.x * s) + (offset.y * c) };

        rotated + origin
    }
}

impl JsDeserialize for Point {
    fn from_value(value: JsValue) -> Self {
        Point { x: JsDeserialize::from_value(value.get("x")), y: JsDeserialize::from_value(value.get("y")) }
    }
}

impl JsSerialize for Point {
    fn to_value(value: Self) -> JsValue {
        let object = js_sys::Object::new();

        object.set("x", JsSerialize::to_value(value.x));
        object.set("y", JsSerialize::to_value(value.y));

        return object.into();
    }
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl std::ops::Div for Point {
    type Output = Point;

    fn div(self, rhs: Self) -> Self::Output {
        Self::Output { x: self.x / rhs.x, y: self.y / rhs.y }
    }
}

impl std::ops::Mul for Point {
    type Output = Point;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output { x: self.x * rhs.x, y: self.y * rhs.y }
    }
}

/*impl From<rapier2d::na::Point2<f32>> for Point {
    fn from(value: rapier2d::na::Point2<f32>) -> Self {
        Self { x: value.x, y: value.y }
    }
}

impl From<Point> for rapier2d::na::Point2<f32> {
    fn from(value: Point) -> Self {
        Self::new(value.x, value.y)
    }
}*/
