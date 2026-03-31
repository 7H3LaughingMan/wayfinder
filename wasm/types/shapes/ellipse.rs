use crate::types::{pixi::JsEllipse, shapes::Rectangle};

#[derive(Clone, Copy, Debug)]
pub struct Ellipse {
    pub x: f64,
    pub y: f64,
    pub half_width: f64,
    pub half_height: f64,
}

impl Ellipse {
    pub fn new(x: impl Into<f64>, y: impl Into<f64>, half_width: impl Into<f64>, half_height: impl Into<f64>) -> Self {
        Ellipse { x: x.into(), y: y.into(), half_width: half_width.into(), half_height: half_height.into() }
    }

    pub fn contains(&self, x: impl Into<f64>, y: impl Into<f64>) -> bool {
        let x: f64 = x.into();
        let y: f64 = y.into();

        if self.half_width <= 0.0 || self.half_height <= 0.0 {
            return false;
        }

        let mut norm_x = (x - self.x) / self.half_width;
        let mut norm_y = (y - self.y) / self.half_height;

        norm_x *= norm_x;
        norm_y *= norm_y;

        return norm_x + norm_y <= 1.0;
    }

    pub fn get_bounds(&self) -> Rectangle {
        return Rectangle::new(
            self.x - self.half_width,
            self.y - self.half_height,
            self.half_width * 2.0,
            self.half_height * 2.0,
        );
    }
}

impl From<JsEllipse> for Ellipse {
    fn from(value: JsEllipse) -> Self {
        Ellipse::new(value.x(), value.y(), value.width(), value.height())
    }
}
