use crate::{
    modules::math,
    types::{Point, Rectangle},
};

#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct Ray {
    pub a: Point,
    pub b: Point,
    x0: f64,
    y0: f64,
    dx: f64,
    dy: f64,
    slope: f64,
    angle: Option<f64>,
    distance: Option<f64>,
}

impl Ray {
    pub fn new(a: Point, b: Point) -> Self {
        let x0 = a.x;
        let y0 = a.y;
        let dx = b.x - a.x;
        let dy = b.y - a.y;
        let slope = dy / dx;

        Self { a, b, x0, y0, dx, dy, slope, angle: None, distance: None }
    }

    pub fn angle(mut self) -> f64 {
        if self.angle.is_none() {
            self.angle = Some(f64::atan2(self.dy, self.dx));
        }
        return self.angle.unwrap();
    }

    pub fn bounds(self) -> Rectangle {
        Rectangle::new(self.x0, self.y0, self.dx, self.dy).normalize()
    }

    pub fn distance(mut self) -> f64 {
        if self.distance.is_none() {
            self.distance = Some(f64::hypot(self.dx, self.dy));
        }
        return self.distance.unwrap();
    }

    pub fn from_angle(x: f64, y: f64, radians: f64, distance: f64) -> Self {
        let x0 = x;
        let y0 = y;
        let dx = radians.cos();
        let dy = radians.sin();
        let slope = dy / dx;
        let angle = math::normalize_radians(radians);
        let distance = distance;

        return Self {
            a: Point { x, y },
            b: Point { x: x + (dx * distance), y: y + (dy * distance) },
            x0,
            y0,
            dx,
            dy,
            slope,
            angle: Some(angle),
            distance: Some(distance),
        };
    }

    pub fn from_arrays(a: [f64; 2], b: [f64; 2]) -> Self {
        Self::new(Point { x: a[0], y: a[1] }, Point { x: b[0], y: b[1] })
    }

    pub fn project(self, t: f64) -> Point {
        Point { x: self.x0 + (t * self.dx), y: self.y0 + (t * self.dy) }
    }
}
