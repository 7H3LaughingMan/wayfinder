use crate::types::helpers::JsObject;
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(
        extends = Object,
        js_name = RegionBehavior,
        js_namespace = ["foundry", "documents"],
        typescript_type = "foundry.documents.RegionBehavior"
    )]
    pub type JsRegionBehavior;

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &JsRegionBehavior) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn name(this: &JsRegionBehavior) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn r#type(this: &JsRegionBehavior) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn system(this: &JsRegionBehavior) -> JsObject;

    #[wasm_bindgen(method, getter)]
    pub fn disabled(this: &JsRegionBehavior) -> bool;
}
