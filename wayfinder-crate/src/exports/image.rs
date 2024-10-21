use crate::{
    log,
    types::{Pixel, Point, Rectangle},
};

pub struct Image {
    pub pixels: Vec<Pixel>,
    pub bounds: Rectangle,
    pub scaled_bounds: Rectangle,
    pub scale: f32,
}

impl Image {
    pub fn new(pixels: Vec<Pixel>, bounds: Rectangle, scaled_bounds: Rectangle) -> Self {
        Self { pixels, bounds, scaled_bounds, scale: scaled_bounds.width / bounds.width }
    }

    pub fn check_pixel(&self, Point { x, y }: Point) -> bool {
        let x = ((x - self.bounds.x) * self.scale) as i32;
        let y = ((y - self.bounds.y) * self.scale) as i32;

        self.get_pixel(x, y).a >= 127
    }

    fn get_pixel(&self, x: i32, y: i32) -> Pixel {
        if !self.scaled_bounds.contains(Point { x: x as f32, y: y as f32 }) {
            return Pixel { a: 0, r: 0, g: 0, b: 0 };
        }

        let idx = (y * self.scaled_bounds.width as i32 + x) as usize;

        if idx > self.pixels.len() {
            return Pixel { a: 0, r: 0, g: 0, b: 0 };
        }

        self.pixels[idx]
    }
}
