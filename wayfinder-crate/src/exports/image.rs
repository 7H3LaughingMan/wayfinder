use web_sys::WebGl2RenderingContext;

use crate::{
    log,
    traits::JsDeserialize,
    types::{GLTexture, Pixel, Point, Rectangle},
};

use super::wayfinder::JsGLTexture;

pub struct Image {
    pub pixels: Vec<Pixel>,
    pub bounds: Rectangle,
    pub scale: f64,
    pub width: i32,
    pub height: i32,
}

impl Image {
    pub fn new(gl: WebGl2RenderingContext, gl_texture: GLTexture, bounds: Rectangle) -> Self {
        let mut data: Vec<u8> = vec![0; (4 * (gl_texture.width * gl_texture.height)) as usize];

        let frame_buffer = gl.create_framebuffer();
        gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, frame_buffer.as_ref());
        gl.framebuffer_texture_2d(
            WebGl2RenderingContext::FRAMEBUFFER,
            WebGl2RenderingContext::COLOR_ATTACHMENT0,
            WebGl2RenderingContext::TEXTURE_2D,
            Some(&gl_texture.texture),
            0,
        );

        if gl.check_framebuffer_status(WebGl2RenderingContext::FRAMEBUFFER)
            == WebGl2RenderingContext::FRAMEBUFFER_COMPLETE
        {
            let _ = gl.read_pixels_with_opt_u8_array(
                0,
                0,
                gl_texture.width as i32,
                gl_texture.height as i32,
                WebGl2RenderingContext::RGBA,
                WebGl2RenderingContext::UNSIGNED_BYTE,
                Some(data.as_mut_slice()),
            );
        }

        gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);

        Self {
            pixels: bytemuck::allocation::cast_vec(data),
            bounds,
            scale: gl_texture.width as f64 / bounds.width,
            width: gl_texture.width as i32,
            height: gl_texture.height as i32,
        }
    }

    pub fn check_pixel(&self, Point { x, y }: Point) -> bool {
        let x = ((x - self.bounds.x) * self.scale) as i32;
        let y = ((y - self.bounds.y) * self.scale) as i32;

        self.get_pixel(x, y).a >= 127
    }

    fn get_pixel(&self, x: i32, y: i32) -> Pixel {
        if x > self.width || y > self.height {
            return Pixel { a: 0, r: 0, g: 0, b: 0 };
        }

        let idx = (y * self.width as i32 + x) as usize;

        if idx > self.pixels.len() {
            return Pixel { a: 0, r: 0, g: 0, b: 0 };
        }

        self.pixels[idx]
    }
}
