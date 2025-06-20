use crate::types::{GLTexture, Point, Rectangle};
use web_sys::WebGl2RenderingContext;

pub struct Fog {
    pub pixels: Vec<u8>,
    pub bounds: Rectangle,
    pub resolution: f64,
    pub width: i32,
    pub height: i32,
}

impl Fog {
    pub fn new(gl: WebGl2RenderingContext, gl_texture: GLTexture, bounds: Rectangle, resolution: f64) -> Self {
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
                gl_texture.width,
                gl_texture.height,
                WebGl2RenderingContext::RGBA,
                WebGl2RenderingContext::UNSIGNED_BYTE,
                Some(data.as_mut_slice()),
            );
        }

        gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);

        Self {
            pixels: data.into_iter().step_by(4).collect(),
            bounds,
            resolution,
            width: gl_texture.width as i32,
            height: gl_texture.height as i32,
        }
    }

    pub fn is_point_explored(&self, Point { mut x, mut y }: Point) -> bool {
        if !self.bounds.contains(x, y) {
            return false;
        }

        x -= self.bounds.x;
        y -= self.bounds.y;

        let x1 = (x * self.resolution).floor() as i32;
        let x0 = if x1 > 0 { x1 - 1 } else { 0 };
        let x2 = if x1 < self.width { x1 + 2 } else { self.width };

        let y1 = (y * self.resolution).floor() as i32;
        let y0 = if y1 > 0 { y1 - 1 } else { 0 };
        let y2 = if y1 < self.height { y1 + 2 } else { self.height };

        for y in y0..y2 {
            let k = y * self.width;
            for x in x0..x2 {
                if self.pixels[(k + x) as usize] != 0 {
                    return true;
                }
            }
        }

        return false;
    }
}
