use crate::{
    CANVAS,
    types::{
        foundry::canvas::JsCanvas,
        helpers::JsObject,
        pixi::JsMatrix,
        wayfinder::{GridOffset2D, GridOffset3D, Point, SceneDimensions, grid::Grid},
    },
};
use bitvec::vec::BitVec;
use std::fmt::Debug;
use web_sys::WebGl2RenderingContext;

pub struct FogManager {
    pub explored: BitVec,
    pub rows: i32,
    pub columns: i32,
}

impl FogManager {
    pub fn new() -> Self {
        let grid = Grid::new();
        let scene_dimensions: SceneDimensions = CANVAS.scene().unwrap().dimensions().into();
        let renderer = CANVAS.app().renderer();
        let gl = renderer.gl();
        let pack_alignment = gl.get_parameter(WebGl2RenderingContext::PACK_ALIGNMENT).unwrap().as_f64().unwrap() as i32;

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
        renderer.render(
            CANVAS.fog().sprite().into(),
            JsObject::new()
                .set("renderTexture", &render_texture)
                .set(
                    "transform",
                    JsMatrix::new(1.0, 0.0, 0.0, 1.0, -CANVAS.fog().sprite().x(), -CANVAS.fog().sprite().y()),
                )
                .into(),
        );

        let framebuffer = render_texture.framebuffer();
        let gl_framebuffer = framebuffer.gl_framebuffers().get(renderer.CONTEXT_UID()).unwrap();
        let resolution = render_texture.resolution();
        let (width, height) = (framebuffer.width() as i32, framebuffer.height() as i32);
        let (real_width, real_height) = (num::Integer::next_multiple_of(&width, &pack_alignment), height);
        let mut pixels = vec![0_u8; (real_width * real_height) as usize];

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

        let is_point_explored = |Point { mut x, mut y }: Point| {
            if !scene_dimensions.scene_rect.contains(x, y) {
                return false;
            }

            x -= scene_dimensions.scene_rect.x;
            y -= scene_dimensions.scene_rect.y;

            let x1 = (x * resolution).floor() as i32;
            let x0 = if x1 > 0 { x1 - 1 } else { 0 };
            let x2 = if x1 < width { x1 + 1 } else { width };

            let y1 = (y * resolution).floor() as i32;
            let y0 = if y1 > 0 { y1 - 1 } else { 0 };
            let y2 = if y1 < height { y1 + 1 } else { height };

            for y in y0..=y2 {
                let k = y * real_width;
                for x in x0..=x2 {
                    if pixels[(k + x) as usize] != 0 {
                        return true;
                    }
                }
            }

            false
        };

        let rows = scene_dimensions.rows;
        let columns = scene_dimensions.columns;
        let mut explored = BitVec::repeat(false, (rows * columns) as usize);

        for j in 0..columns {
            for i in 0..rows {
                explored.set(
                    ((i * columns) + j) as usize,
                    is_point_explored(grid.get_center_point(GridOffset3D { i, j, k: 0 }).into()),
                );
            }
        }

        FogManager { explored, rows, columns }
    }

    pub fn is_offset_explored(&self, GridOffset2D { i, j }: GridOffset2D) -> bool {
        if i < 0 || j < 0 || i >= self.rows || j >= self.columns {
            false
        } else {
            self.explored[((i * self.columns) + j) as usize]
        }
    }
}

impl Debug for FogManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FogManager")
            .field("explored", &self.explored.clone().into_vec())
            .field("rows", &self.rows)
            .field("columns", &self.columns)
            .finish()
    }
}
