use crate::modules::number;
use crate::traits::{JsDeserialize, JsHelper, JsSerialize};
use std::cmp::Ordering;
use std::hash::Hash;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct ElevatedPoint {
    pub x: f64,
    pub y: f64,
    pub elevation: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn close(&self, other: Point) -> bool {
        number::between(self.x, other.x - 1e-8, other.x + 1e-8, None)
            && number::between(self.y, other.y - 1e-8, other.y + 1e-8, None)
    }

    pub fn round(&self) -> Point {
        Point { x: self.x.round(), y: self.y.round() }
    }
}

impl ElevatedPoint {
    pub fn new(x: f64, y: f64, elevation: f64) -> Self {
        Self { x, y, elevation }
    }

    pub fn close(&self, other: ElevatedPoint) -> bool {
        number::between(self.x, other.x - 1e-8, other.x + 1e-8, None)
            && number::between(self.y, other.y - 1e-8, other.y + 1e-8, None)
            && number::between(self.elevation, other.elevation - 1e-8, other.elevation + 1e-8, None)
    }

    pub fn round(&self) -> ElevatedPoint {
        ElevatedPoint { x: self.x.round(), y: self.y.round(), elevation: self.elevation.round() }
    }
}

impl From<ElevatedPoint> for Point {
    fn from(ElevatedPoint { x, y, elevation: _ }: ElevatedPoint) -> Self {
        Point { x, y }
    }
}

impl From<Point> for ElevatedPoint {
    fn from(Point { x, y }: Point) -> Self {
        ElevatedPoint { x, y, elevation: 0.0 }
    }
}

impl JsDeserialize for Point {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        Point { x: data.get_value("x"), y: data.get_value("y") }
    }
}

impl JsDeserialize for ElevatedPoint {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        ElevatedPoint { x: data.get_value("x"), y: data.get_value("y"), elevation: data.get_value("elevation") }
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

impl JsSerialize for ElevatedPoint {
    fn to_value(value: Self) -> wasm_bindgen::JsValue {
        let object = js_sys::Object::new();

        object.set("x", JsSerialize::to_value(value.x));
        object.set("y", JsSerialize::to_value(value.y));
        object.set("elevation", JsSerialize::to_value(value.elevation));

        return object.into();
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
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
