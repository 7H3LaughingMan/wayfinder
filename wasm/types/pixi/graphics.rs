use crate::types::pixi::{JsCircle, JsEllipse, JsPolygon, JsRectangle, JsRoundedRectangle};
use js_sys::{Array, Number, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = Object,
        js_name = Graphics,
        js_namespace = PIXI,
        typescript_type = "PIXI.Graphics"
    )]
    pub type JsGraphics;

    #[wasm_bindgen(js_class = Graphics, js_namespace = PIXI, constructor)]
    pub fn new() -> JsGraphics;

    /// Specifies a simple one-color fill that subsequent calls to other Graphics methods (such as lineTo() or drawCircle()) use when drawing.
    #[wasm_bindgen(method, js_name = beginFill)]
    pub fn begin_fill(this: &JsGraphics, color: u32, alpha: f64) -> JsGraphics;

    /// Begin adding holes to the last draw shape IMPORTANT: holes must be fully inside a shape to work Also weirdness ensues if holes overlap! Ellipses, Circles, Rectangles and Rounded Rectangles cannot be holes or host for holes in CanvasRenderer, please use moveTo lineTo, quadraticCurveTo if you rely on pixi-legacy bundle.
    #[wasm_bindgen(method, js_name = beginHole)]
    pub fn begin_hole(this: &JsGraphics) -> JsGraphics;

    /// Clears the graphics that were drawn to this Graphics object, and resets fill and line style settings.
    #[wasm_bindgen(method)]
    pub fn clear(this: &JsGraphics) -> JsGraphics;

    /// Draws a circle.
    #[wasm_bindgen(method, js_name = drawCircle)]
    pub fn draw_circle(this: &JsGraphics, x: f64, y: f64, radius: f64) -> JsGraphics;

    /// Draws an ellipse.
    #[wasm_bindgen(method, js_name = drawEllipse)]
    pub fn draw_ellipse(this: &JsGraphics, x: f64, y: f64, width: f64, height: f64) -> JsGraphics;

    /// Draw a polygon using the given path.
    #[wasm_bindgen(method, js_name = drawPolygon)]
    pub fn draw_polygon(this: &JsGraphics, path: Array<Number>) -> JsGraphics;

    /// Draws a rectangle shape.
    #[wasm_bindgen(method, js_name = drawRect)]
    pub fn draw_rect(this: &JsGraphics, x: f64, y: f64, width: f64, height: f64) -> JsGraphics;

    /// Draw a rectangle shape with rounded/beveled corners.
    #[wasm_bindgen(method, js_name = drawRoundedRect)]
    pub fn draw_rounded_rect(this: &JsGraphics, x: f64, y: f64, width: f64, height: f64, radius: f64) -> JsGraphics;

    /// Draw any shape.
    #[wasm_bindgen(method, js_name = drawShape)]
    pub fn draw_shape_circle(this: &JsGraphics, shape: JsCircle) -> JsGraphics;

    /// Draw any shape.
    #[wasm_bindgen(method, js_name = drawShape)]
    pub fn draw_shape_ellipse(this: &JsGraphics, shape: JsEllipse) -> JsGraphics;

    /// Draw any shape.
    #[wasm_bindgen(method, js_name = drawShape)]
    pub fn draw_shape_polygon(this: &JsGraphics, shape: JsPolygon) -> JsGraphics;

    /// Draw any shape.
    #[wasm_bindgen(method, js_name = drawShape)]
    pub fn draw_shape_rectangle(this: &JsGraphics, shape: JsRectangle) -> JsGraphics;

    /// Draw any shape.
    #[wasm_bindgen(method, js_name = drawShape)]
    pub fn draw_shape_rounded_rectangle(this: &JsGraphics, shape: JsRoundedRectangle) -> JsGraphics;

    /// Applies a fill to the lines and shapes that were added since the last call to the beginFill() method.
    #[wasm_bindgen(method, js_name = endFill)]
    pub fn end_fill(this: &JsGraphics) -> JsGraphics;

    /// End adding holes to the last draw shape.
    #[wasm_bindgen(method, js_name = endHole)]
    pub fn end_hole(this: &JsGraphics) -> JsGraphics;
}
