use crate::{
    CANVAS,
    types::{foundry::canvas::JsCanvas, helpers::JsObject, wayfinder::Point},
};
use geo::{Contains, Coord, Rect};
use std::fmt::Debug;
use web_sys::WebGl2RenderingContext;

pub struct FogManager {
    pub pixels: Vec<u8>,
    pub resolution: f64,
    pub scene_rect: Rect,
    pub width: i32,
    pub height: i32,
}

impl FogManager {
    pub fn new() -> Self {
        let renderer = CANVAS.app().renderer();
        let gl = renderer.gl();

        let render_texture = JsCanvas::get_render_texture(
            JsObject::new()
                .set(
                    "clearColor",
                    js_sys::ArrayTuple::new4(
                        &js_sys::Number::from(0),
                        &js_sys::Number::from(0),
                        &js_sys::Number::from(0),
                        &js_sys::Number::from(1),
                    ),
                )
                .set("textureConfiguration", CANVAS.fog().texture_configuration())
                .into(),
        );
        renderer.render(CANVAS.fog().sprite().into(), JsObject::new().set("renderTexture", &render_texture).into());

        let framebuffer = render_texture.framebuffer();
        let gl_framebuffer = framebuffer.gl_framebuffers().get(renderer.CONTEXT_UID()).unwrap();

        let scene_rect: Rect = CANVAS.scene().unwrap().dimensions().scene_rect().into();
        let resolution = render_texture.resolution();
        let width = framebuffer.width() as i32;
        let height = framebuffer.height() as i32;
        let mut pixels = vec![0_u8; (framebuffer.width() * framebuffer.height()) as usize];

        gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, Option::Some(&gl_framebuffer.framebuffer()));

        let _ = gl.read_pixels_with_opt_u8_array(
            0,
            0,
            width,
            height,
            WebGl2RenderingContext::RED,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            Some(pixels.as_mut_slice()),
        );

        gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);
        render_texture.destroy(Some(true));

        FogManager { pixels, resolution, scene_rect, width, height }
    }

    pub fn is_point_explored(&self, Point { mut x, mut y }: Point) -> bool {
        if !self.scene_rect.contains(&(Coord { x, y })) {
            return false;
        }

        x -= self.scene_rect.min().x;
        y -= self.scene_rect.min().y;

        let x1 = (x * self.resolution).floor() as i32;
        let x0 = if x1 > 0 { x1 - 1 } else { 0 };
        let x2 = if x1 < self.width { x1 + 1 } else { self.width };

        let y1 = (y * self.resolution).floor() as i32;
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
            .field("resolution", &self.resolution)
            .field("scene_rect", &self.scene_rect)
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}
