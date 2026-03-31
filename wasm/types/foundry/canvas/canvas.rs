use crate::types::{
    foundry::{
        canvas::{
            layers::{JsControlsLayer, JsRegionLayer, JsWallsLayer},
            perception::JsFogManager,
        },
        grid::JsBaseGrid,
    },
    pixi::{JsApplication, JsRectangle},
};
use js_sys::{JsOption, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(
        extends = Object,
        js_name = Canvas,
        js_namespace = ["foundry", "canvas"],
        typescript_type = "foundry.canvas.Canvas"
    )]
    pub type JsCanvas;

    #[wasm_bindgen(method, getter)]
    pub fn app(this: &JsCanvas) -> JsApplication;

    #[wasm_bindgen(method, getter)]
    pub fn fog(this: &JsCanvas) -> JsFogManager;

    #[wasm_bindgen(method, getter)]
    pub fn dimensions(this: &JsCanvas) -> JsOption<JsCanvasDimensions>;

    #[wasm_bindgen(method, getter)]
    pub fn grid(this: &JsCanvas) -> JsOption<JsBaseGrid>;

    #[wasm_bindgen(method, getter)]
    pub fn controls(this: &JsCanvas) -> JsControlsLayer;

    #[wasm_bindgen(method, getter)]
    pub fn regions(this: &JsCanvas) -> JsRegionLayer;

    #[wasm_bindgen(method, getter)]
    pub fn walls(this: &JsCanvas) -> JsWallsLayer;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen]
    pub type JsCanvasDimensions;

    #[wasm_bindgen(method, getter)]
    pub fn width(this: &JsCanvasDimensions) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn height(this: &JsCanvasDimensions) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn size(this: &JsCanvasDimensions) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn rect(this: &JsCanvasDimensions) -> JsRectangle;

    #[wasm_bindgen(method, getter = sceneX)]
    pub fn scene_x(this: &JsCanvasDimensions) -> f64;

    #[wasm_bindgen(method, getter = sceneY)]
    pub fn scene_y(this: &JsCanvasDimensions) -> f64;

    #[wasm_bindgen(method, getter = sceneWidth)]
    pub fn scene_width(this: &JsCanvasDimensions) -> f64;

    #[wasm_bindgen(method, getter = sceneHeight)]
    pub fn scene_height(this: &JsCanvasDimensions) -> f64;

    #[wasm_bindgen(method, getter = sceneRect)]
    pub fn scene_rect(this: &JsCanvasDimensions) -> JsRectangle;

    #[wasm_bindgen(method, getter)]
    pub fn distance(this: &JsCanvasDimensions) -> f64;

    #[wasm_bindgen(method, getter = distancePixels)]
    pub fn distance_pixels(this: &JsCanvasDimensions) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn units(this: &JsCanvasDimensions) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn ratio(this: &JsCanvasDimensions) -> f64;

    #[wasm_bindgen(method, getter = maxR)]
    pub fn max_r(this: &JsCanvasDimensions) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn rows(this: &JsCanvasDimensions) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn columns(this: &JsCanvasDimensions) -> f64;
}
