use js_sys::Object;
use wasm_bindgen::prelude::*;
use crate::types::foundry::canvas::containers::JsSpriteMesh;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(
        extends = Object, 
        js_name = FogManager, 
        js_namespace = ["foundry", "canvas", "perception"],
        typescript_type = "foundry.canvas.perception.FogManager"
    )]
    pub type JsFogManager;

    #[wasm_bindgen(method, getter)]
    pub fn sprite(this: &JsFogManager) -> JsSpriteMesh;

    #[wasm_bindgen(method, getter = textureConfiguration)]
    pub fn texture_configuration(this: &JsFogManager) -> JsCanvasVisibilityTextureConfiguration;

    #[wasm_bindgen(method, getter = tokenVision)]
    pub fn token_vision(this: &JsFogManager) -> bool;

    #[wasm_bindgen(method, getter = fogExploration)]
    pub fn fog_exploration(this: &JsFogManager) -> bool;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen]
    pub type JsCanvasVisibilityTextureConfiguration;

    #[wasm_bindgen(method, getter)]
    pub fn resolution(this: &JsCanvasVisibilityTextureConfiguration) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn width(this: &JsCanvasVisibilityTextureConfiguration) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn height(this: &JsCanvasVisibilityTextureConfiguration) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn mipmap(this: &JsCanvasVisibilityTextureConfiguration) -> f64;

    #[wasm_bindgen(method, getter = scaleMode)]
    pub fn scale_mode(this: &JsCanvasVisibilityTextureConfiguration) -> f64;

    #[wasm_bindgen(method, getter = alphaMode)]
    pub fn alpha_mode(this: &JsCanvasVisibilityTextureConfiguration) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn multisample(this: &JsCanvasVisibilityTextureConfiguration) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn format(this: &JsCanvasVisibilityTextureConfiguration) -> f64;
}
