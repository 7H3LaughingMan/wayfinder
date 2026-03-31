use crate::types::{pixi::JsCircle, shapes::Rectangle};

#[derive(Clone, Copy, Debug)]
pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

impl Circle {
    pub fn new(x: impl Into<f64>, y: impl Into<f64>, radius: impl Into<f64>) -> Self {
        Circle { x: x.into(), y: y.into(), radius: radius.into() }
    }

    pub fn contains(&self, x: impl Into<f64>, y: impl Into<f64>) -> bool {
        let x: f64 = x.into();
        let y: f64 = y.into();

        if self.radius <= 0.0 {
            return false;
        }

        let r2 = self.radius * self.radius;
        let mut dx = self.x - x;
        let mut dy = self.y - y;

        dx *= dx;
        dy *= dy;

        return dx + dy <= r2;
    }

    pub fn get_bounds(&self) -> Rectangle {
        return Rectangle::new(self.x - self.radius, self.y - self.radius, self.radius * 2.0, self.radius * 2.0);
    }
}

impl From<JsCircle> for Circle {
    fn from(value: JsCircle) -> Self {
        Circle::new(value.x(), value.y(), value.radius())
    }
}
