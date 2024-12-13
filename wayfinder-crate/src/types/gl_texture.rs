use crate::traits::{JsDeserialize, JsHelper};
use web_sys::WebGlTexture;

#[derive(Clone)]
#[derive(Debug)]
pub struct GLTexture {
    pub texture: WebGlTexture,
    pub width: u32,
    pub height: u32,
    pub mipmap: bool,
    pub wrap_mode: u32,
    pub r#type: u32,
    pub internal_format: u32,
    pub sampler_type: u32,
    pub dirty_id: u32,
    pub dirty_style_id: u32,
}

impl JsDeserialize for GLTexture {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        Self {
            texture: data.get("texture").into(),
            width: data.get_value("width"),
            height: data.get_value("height"),
            mipmap: data.get_value("mipmap"),
            wrap_mode: data.get_value("wrapMode"),
            r#type: data.get_value("type"),
            internal_format: data.get_value("internalFormat"),
            sampler_type: data.get_value("samplerType"),
            dirty_id: data.get_value("dirtyId"),
            dirty_style_id: data.get_value("dirtyStyleId"),
        }
    }
}
