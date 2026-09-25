use crate::types::foundry::{JsElevatedPoint, JsPoint};

#[derive(Clone, Copy, Debug, Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: impl Into<f64>, y: impl Into<f64>) -> Self {
        Self { x: x.into(), y: y.into() }
    }

    pub fn round(&self) -> Point {
        Point { x: self.x.round(), y: self.y.round() }
    }
}

impl From<JsPoint> for Point {
    fn from(value: JsPoint) -> Self {
        Point { x: value.x(), y: value.y() }
    }
}

impl From<JsPoint> for geo::Coord {
    fn from(value: JsPoint) -> Self {
        geo::Coord { x: value.x(), y: value.y() }
    }
}

impl From<ElevatedPoint> for Point {
    fn from(ElevatedPoint { x, y, elevation: _ }: ElevatedPoint) -> Self {
        Point { x, y }
    }
}

impl From<Point> for geo::Coord {
    fn from(Point { x, y }: Point) -> Self {
        geo::Coord { x, y }
    }
}

impl From<Point> for geo::Point {
    fn from(Point { x, y }: Point) -> Self {
        geo::Point(geo::Coord { x, y })
    }
}

impl From<geo::Coord> for Point {
    fn from(geo::Coord { x, y }: geo::Coord) -> Self {
        Point { x, y }
    }
}

impl From<geo::Point> for Point {
    fn from(value: geo::Point) -> Self {
        Point { x: value.x(), y: value.y() }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ElevatedPoint {
    pub x: f64,
    pub y: f64,
    pub elevation: f64,
}

impl ElevatedPoint {
    pub fn new(x: impl Into<f64>, y: impl Into<f64>, elevation: impl Into<f64>) -> Self {
        Self { x: x.into(), y: y.into(), elevation: elevation.into() }
    }

    pub fn round(&self) -> ElevatedPoint {
        ElevatedPoint { x: self.x.round(), y: self.y.round(), elevation: self.elevation.round() }
    }
}

impl From<JsPoint> for ElevatedPoint {
    fn from(value: JsPoint) -> Self {
        ElevatedPoint { x: value.x(), y: value.y(), elevation: 0.0 }
    }
}

impl From<JsElevatedPoint> for ElevatedPoint {
    fn from(value: JsElevatedPoint) -> Self {
        ElevatedPoint { x: value.x(), y: value.y(), elevation: value.elevation() }
    }
}

impl From<Point> for ElevatedPoint {
    fn from(Point { x, y }: Point) -> Self {
        ElevatedPoint { x, y, elevation: 0.0 }
    }
}

impl From<ElevatedPoint> for geo::Coord {
    fn from(ElevatedPoint { x, y, elevation: _ }: ElevatedPoint) -> Self {
        geo::Coord { x, y }
    }
}

impl From<ElevatedPoint> for geo::Point {
    fn from(ElevatedPoint { x, y, elevation: _ }: ElevatedPoint) -> Self {
        geo::Point(geo::Coord { x, y })
    }
}

impl From<geo::Coord> for ElevatedPoint {
    fn from(geo::Coord { x, y }: geo::Coord) -> Self {
        ElevatedPoint { x, y, elevation: 0.0 }
    }
}

impl From<geo::Point> for ElevatedPoint {
    fn from(value: geo::Point) -> Self {
        ElevatedPoint { x: value.x(), y: value.y(), elevation: 0.0 }
    }
}
