use bytemuck::{Pod, Zeroable};

#[derive(Pod, Zeroable)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
#[derive(Debug)]
#[repr(C)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Pixel {
    pub fn average(pixels: Vec<Pixel>) -> Pixel {
        let count = pixels.len() as u32;

        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        let mut a = 0;

        for pixel in pixels {
            r += pixel.r as u32;
            g += pixel.g as u32;
            b += pixel.b as u32;
            a += pixel.a as u32;
        }

        return Pixel { r: (r / count) as u8, g: (g / count) as u8, b: (b / count) as u8, a: (a / count) as u8 };
    }
}
