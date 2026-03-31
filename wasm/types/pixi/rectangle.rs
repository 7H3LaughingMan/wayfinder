use crate::types::shapes::Rectangle;
use geo::Rect;
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = Object,
        js_name = Rectangle,
        js_namespace = PIXI,
        typescript_type = "PIXI.Rectangle"
    )]
    pub type JsRectangle;

    #[wasm_bindgen(constructor, js_class = Rectangle, js_namespace = PIXI)]
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> JsRectangle;

    #[wasm_bindgen(static_method_of = JsRectangle, js_class = Rectangle, js_namespace = PIXI, getter = EMPTY)]
    pub fn empty() -> JsRectangle;

    #[wasm_bindgen(method, getter)]
    pub fn bottom(this: &JsRectangle) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn height(this: &JsRectangle) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn left(this: &JsRectangle) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn right(this: &JsRectangle) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn top(this: &JsRectangle) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn width(this: &JsRectangle) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn x(this: &JsRectangle) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn y(this: &JsRectangle) -> f64;

    #[wasm_bindgen(method)]
    pub fn ceil(this: &JsRectangle, resolution: f64, eps: f64) -> JsRectangle;

    #[wasm_bindgen(method)]
    pub fn clone(this: &JsRectangle) -> JsRectangle;

    #[wasm_bindgen(method)]
    pub fn contains(this: &JsRectangle, x: f64, y: f64) -> bool;

    #[wasm_bindgen(method, js_name = copyFrom)]
    pub fn copy_from(this: &JsRectangle, rectangle: JsRectangle) -> JsRectangle;

    #[wasm_bindgen(method, js_name = copyTo)]
    pub fn copy_to(this: &JsRectangle, rectangle: JsRectangle) -> JsRectangle;

    #[wasm_bindgen(method)]
    pub fn enlarge(this: &JsRectangle, rectangle: JsRectangle) -> JsRectangle;

    #[wasm_bindgen(method)]
    pub fn fit(this: &JsRectangle, rectangle: JsRectangle) -> JsRectangle;

    #[wasm_bindgen(method)]
    pub fn intersects(this: &JsRectangle, other: JsRectangle) -> bool;

    #[wasm_bindgen(method)]
    pub fn pad(this: &JsRectangle, padding_x: f64, padding_y: f64) -> JsRectangle;
}

impl From<Rectangle> for JsRectangle {
    fn from(value: Rectangle) -> Self {
        JsRectangle::new(value.x, value.y, value.width, value.height)
    }
}

impl From<Rect> for JsRectangle {
    fn from(value: Rect) -> Self {
        JsRectangle::new(value.min().x, value.min().y, value.width(), value.height())
    }
}

impl From<JsRectangle> for Rect {
    fn from(value: JsRectangle) -> Self {
        Rect::new((value.left(), value.top()), (value.right(), value.bottom()))
    }
}
