use crate::{
    CANVAS,
    types::{Point, helpers::JsObject},
};
use geo::{Contains, Coord, Rect};
use std::fmt::Debug;
use web_sys::WebGl2RenderingContext;

const RESOLUTION: f64 = 0.25;

pub struct FogManager {
    pub pixels: Vec<u8>,
    pub scene_rect: Rect,
    pub width: i32,
    pub height: i32,
}

impl FogManager {
    pub fn new() -> Self {
        let scene_rect: Rect = CANVAS.dimensions().unwrap().scene_rect().into();

        let renderer = CANVAS.app().renderer();
        let gl = renderer.gl();

        let render_texture = renderer
            .generate_texture(CANVAS.fog().sprite().into(), JsObject::new().set("resolution", RESOLUTION).into());
        let framebuffer = render_texture.framebuffer();
        let gl_framebuffer = framebuffer.gl_framebuffers().get(renderer.CONTEXT_UID()).unwrap();

        let mut data = vec![0_u8; (4.0 * (framebuffer.width() * framebuffer.height())) as usize];

        gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, Option::Some(&gl_framebuffer.framebuffer()));

        let _ = gl.read_pixels_with_opt_u8_array(
            0,
            0,
            framebuffer.width() as i32,
            framebuffer.height() as i32,
            WebGl2RenderingContext::RGBA,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            Some(data.as_mut_slice()),
        );

        gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);

        FogManager {
            pixels: data.into_iter().step_by(4).collect(),
            scene_rect,
            width: framebuffer.width() as i32,
            height: framebuffer.height() as i32,
        }
    }

    pub fn is_point_explored(&self, Point { mut x, mut y }: Point) -> bool {
        if !self.scene_rect.contains(&(Coord { x, y })) {
            return false;
        }

        x -= self.scene_rect.min().x;
        y -= self.scene_rect.min().y;

        let x1 = (x * RESOLUTION).floor() as i32;
        let x0 = if x1 > 0 { x1 - 1 } else { 0 };
        let x2 = if x1 < self.width { x1 + 1 } else { self.width };

        let y1 = (y * RESOLUTION).floor() as i32;
        let y0 = if y1 > 0 { y1 - 1 } else { 0 };
        let y2 = if y1 < self.height { y1 + 1 } else { self.height };

        for y in y0..=y2 {
            let k = y * self.width;
            for x in x0..=x2 {
                if self.pixels[(k + x) as usize] != 0 {
                    return true;
                }
            }
        }

        false
    }
}

impl Debug for FogManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FogManager")
            .field("pixels", &self.pixels.len())
            .field("scene_rect", &self.scene_rect)
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}
