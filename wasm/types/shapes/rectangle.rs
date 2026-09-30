use crate::types::pixi::JsRectangle;
use geo::Rect;

#[derive(Clone, Copy, Debug)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub const EMPTY: Rectangle = Rectangle { x: 0.0, y: 0.0, width: 0.0, height: 0.0 };

    pub fn new(x: impl Into<f64>, y: impl Into<f64>, width: impl Into<f64>, height: impl Into<f64>) -> Self {
        Rectangle { x: x.into(), y: y.into(), width: width.into(), height: height.into() }
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

    pub fn copy_from(&mut self, rectangle: &Rectangle) {
        self.x = rectangle.x;
        self.y = rectangle.y;
        self.width = rectangle.width;
        self.height = rectangle.height;
    }

    pub fn copy_to(&self, rectangle: &mut Rectangle) {
        rectangle.x = self.x;
        rectangle.y = self.y;
        rectangle.width = self.width;
        rectangle.height = self.height;
    }

    pub fn contains(&self, x: impl Into<f64>, y: impl Into<f64>) -> bool {
        let x: f64 = x.into();
        let y: f64 = y.into();

        if self.width <= 0.0 || self.height <= 0.0 {
            return false;
        }

        x >= self.left() && x < self.right() && y >= self.top() && y < self.bottom()
    }

    pub fn intersects(&self, other: &Rectangle) -> bool {
        let x0 = if self.x < other.x { other.x } else { self.x };
        let x1 = if self.right() > other.right() { other.right() } else { self.right() };

        if x1 <= x0 {
            return false;
        }

        let y0 = if self.y < other.y { other.y } else { self.y };
        let y1 = if self.bottom() > other.bottom() { other.bottom() } else { self.bottom() };

        return y1 >= y0;
    }

    pub fn pad(&mut self, padding_x: f64, padding_y: f64) {
        self.x -= padding_x;
        self.y -= padding_y;
        self.width += padding_x * 2.0;
        self.height += padding_y * 2.0;
    }

    pub fn fit(&mut self, rectangle: &Rectangle) {
        let x1 = f64::max(self.left(), rectangle.left());
        let x2 = f64::min(self.right(), rectangle.right());
        let y1 = f64::max(self.top(), rectangle.top());
        let y2 = f64::min(self.bottom(), rectangle.bottom());

        self.x = x1;
        self.width = f64::max(x2 - x1, 0.0);
        self.y = y1;
        self.height = f64::max(y2 - y1, 0.0);
    }

    pub fn ceil(&mut self, resolution: f64, eps: f64) {
        let x2 = f64::ceil((self.right() - eps) * resolution) / resolution;
        let y2 = f64::ceil((self.bottom() - eps) * resolution) / resolution;

        self.x = f64::floor((self.left() + eps) * resolution) / resolution;
        self.y = f64::floor((self.top() + eps) * resolution) / resolution;

        self.width = x2 - self.x;
        self.height = y2 - self.y;
    }

    pub fn enlarge(&mut self, rectangle: &Rectangle) {
        let x1 = f64::min(self.left(), rectangle.left());
        let x2 = f64::max(self.right(), rectangle.right());
        let y1 = f64::min(self.top(), rectangle.top());
        let y2 = f64::max(self.bottom(), rectangle.bottom());

        self.x = x1;
        self.width = x2 - x1;
        self.y = y1;
        self.height = y2 - y1;
    }
}

impl From<JsRectangle> for Rectangle {
    fn from(value: JsRectangle) -> Self {
        Rectangle { x: value.x(), y: value.y(), width: value.width(), height: value.height() }
    }
}

impl From<Rectangle> for Rect {
    fn from(value: Rectangle) -> Self {
        Rect::new((value.left(), value.top()), (value.right(), value.bottom()))
    }
}

impl From<Rect> for Rectangle {
    fn from(value: Rect) -> Self {
        Rectangle { x: value.min().x, y: value.min().y, width: value.width(), height: value.height() }
    }
}
