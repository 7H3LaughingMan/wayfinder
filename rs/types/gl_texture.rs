use crate::traits::{JsDeserialize, JsHelper};
use web_sys::WebGlTexture;

#[derive(Clone)]
#[derive(Debug)]
pub struct GLTexture {
    pub dirty_id: i32,
    pub dirty_style_id: i32,
    pub height: i32,
    pub internal_format: i32,
    pub mipmap: bool,
    pub sampler_type: i32,
    pub texture: WebGlTexture,
    pub r#type: i32,
    pub width: i32,
    pub wrap_mode: i32,
}

impl JsDeserialize for GLTexture {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        Self {
            dirty_id: data.get_value("dirtyId"),
            dirty_style_id: data.get_value("dirtyStyleId"),
            height: data.get_value("height"),
            internal_format: data.get_value("internalFormat"),
            mipmap: data.get_value("mipmap"),
            sampler_type: data.get_value("samplerType"),
            texture: data.get("texture").into(),
            r#type: data.get_value("type"),
            width: data.get_value("width"),
            wrap_mode: data.get_value("wrapMode"),
        }
    }
}
