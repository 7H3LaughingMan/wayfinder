use js_sys::Object;
use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;

use crate::types::pixi::JsRenderTexture;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = Object,
        js_name = Renderer,
        js_namespace = PIXI,
        typescript_type = "PIXI.Renderer"
    )]
    pub type JsRenderer;

    /// Unique UID assigned to the renderer's WebGL context.
    #[wasm_bindgen(method, getter)]
    pub fn CONTEXT_UID(this: &JsRenderer) -> u32;

    /// WebGL context, set by this.context.
    #[wasm_bindgen(method, getter)]
    pub fn gl(this: &JsRenderer) -> WebGl2RenderingContext;

    /// Useful function that returns a texture of the display object that can then be used to create sprites
    /// This can be quite useful if your displayObject is complicated and needs to be reused multiple times.
    #[wasm_bindgen(method, js_name = generateTexture)]
    pub fn generate_texture(this: &JsRenderer, display_object: JsValue, options: JsValue) -> JsRenderTexture;

    /// Renders the object to its WebGL view.
    #[wasm_bindgen(method)]
    pub fn render(this: &JsRenderer, display_object: JsValue, options: JsValue);
}
