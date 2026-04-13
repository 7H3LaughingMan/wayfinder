use crate::types::foundry::{
    documents::{JsRegionDocument, JsSceneDimensions, JsWallDocument},
    grid::JsBaseGrid,
};
use js_sys::{Map, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(
        extends = Object,
        js_name = Scene,
        js_namespace = ["foundry", "documents"],
        typescript_type = "foundry.documents.Scene"
    )]
    pub type JsScene;

    #[wasm_bindgen(method, getter)]
    pub fn dimensions(this: &JsScene) -> JsSceneDimensions;

    #[wasm_bindgen(method, getter)]
    pub fn grid(this: &JsScene) -> JsBaseGrid;

    #[wasm_bindgen(method, getter)]
    pub fn regions(this: &JsScene) -> Map<String, JsRegionDocument>;

    #[wasm_bindgen(method, getter)]
    pub fn walls(this: &JsScene) -> Map<String, JsWallDocument>;
}
