use crate::types::pixi::JsRectangle;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen]
    pub type JsSceneDimensions;

    #[wasm_bindgen(method, getter)]
    pub fn width(this: &JsSceneDimensions) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn height(this: &JsSceneDimensions) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn size(this: &JsSceneDimensions) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn rect(this: &JsSceneDimensions) -> JsRectangle;

    #[wasm_bindgen(method, getter = sceneX)]
    pub fn scene_x(this: &JsSceneDimensions) -> f64;

    #[wasm_bindgen(method, getter = sceneY)]
    pub fn scene_y(this: &JsSceneDimensions) -> f64;

    #[wasm_bindgen(method, getter = sceneWidth)]
    pub fn scene_width(this: &JsSceneDimensions) -> f64;

    #[wasm_bindgen(method, getter = sceneHeight)]
    pub fn scene_height(this: &JsSceneDimensions) -> f64;

    #[wasm_bindgen(method, getter = sceneRect)]
    pub fn scene_rect(this: &JsSceneDimensions) -> JsRectangle;

    #[wasm_bindgen(method, getter)]
    pub fn distance(this: &JsSceneDimensions) -> f64;

    #[wasm_bindgen(method, getter = distancePixels)]
    pub fn distance_pixels(this: &JsSceneDimensions) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn ratio(this: &JsSceneDimensions) -> f64;

    #[wasm_bindgen(method, getter = maxR)]
    pub fn max_r(this: &JsSceneDimensions) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn rows(this: &JsSceneDimensions) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn columns(this: &JsSceneDimensions) -> f64;
}
