use crate::traits::{JsDeserialize, JsHelper, JsSerialize};
use std::cmp::Ordering;
use std::hash::Hash;
use std::ops::{Add, Div, DivAssign, Mul, MulAssign, Sub};

#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl JsDeserialize for Point {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        Point { x: data.get_value("x"), y: data.get_value("y") }
    }
}

impl JsSerialize for Point {
    fn to_value(value: Self) -> wasm_bindgen::JsValue {
        let object = js_sys::Object::new();

        object.set("x", JsSerialize::to_value(value.x));
        object.set("y", JsSerialize::to_value(value.y));

        return object.into();
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Div for Point {
    type Output = Point;

    fn div(self, rhs: Self) -> Self::Output {
        Self::Output { x: self.x / rhs.x, y: self.y / rhs.y }
    }
}

impl Mul for Point {
    type Output = Point;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output { x: self.x * rhs.x, y: self.y * rhs.y }
    }
}

impl Add<f64> for Point {
    type Output = Point;

    fn add(self, rhs: f64) -> Self::Output {
        Self::Output { x: self.x + rhs, y: self.y + rhs }
    }
}

impl Sub<f64> for Point {
    type Output = Point;

    fn sub(self, rhs: f64) -> Self::Output {
        Self::Output { x: self.x - rhs, y: self.y - rhs }
    }
}

impl Div<f64> for Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Self::Output {
        Self::Output { x: self.x / rhs, y: self.y / rhs }
    }
}

impl DivAssign<f64> for Point {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output { x: self.x * rhs, y: self.y * rhs }
    }
}

impl MulAssign<f64> for Point {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}
