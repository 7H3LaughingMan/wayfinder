use crate::types::{
    foundry::{
        JsPoint,
        canvas::{layers::JsControlsLayer, perception::JsFogManager},
        documents::JsScene,
    },
    pixi::{JsApplication, JsRenderTexture},
};
use js_sys::Object;
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
    pub fn controls(this: &JsCanvas) -> JsControlsLayer;

    #[wasm_bindgen(method, getter)]
    pub fn scene(this: &JsCanvas) -> Option<JsScene>;

    #[wasm_bindgen(method)]
    pub async fn ping(this: &JsCanvas, origin: JsPoint);

    #[wasm_bindgen(static_method_of = JsCanvas, js_namespace = ["foundry", "canvas"], js_class = "Canvas", js_name = "getRenderTexture")]
    pub fn get_render_texture(options: JsValue) -> JsRenderTexture;
}
