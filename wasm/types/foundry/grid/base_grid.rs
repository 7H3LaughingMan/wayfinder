use crate::types::foundry::{GridType, utils::JsColor};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(
        extends = Object,
        js_name = BaseGrid,
        js_namespace = ["foundry", "grid"],
        typescript_type = "foundry.grid.BaseGrid"
    )]
    pub type JsBaseGrid;

    #[wasm_bindgen(method, getter)]
    pub fn size(this: &JsBaseGrid) -> f64;

    #[wasm_bindgen(method, getter = sizeX)]
    pub fn size_x(this: &JsBaseGrid) -> f64;

    #[wasm_bindgen(method, getter = sizeY)]
    pub fn size_y(this: &JsBaseGrid) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn distance(this: &JsBaseGrid) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn units(this: &JsBaseGrid) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn style(this: &JsBaseGrid) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn thickness(this: &JsBaseGrid) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn color(this: &JsBaseGrid) -> JsColor;

    #[wasm_bindgen(method, getter)]
    pub fn alpha(this: &JsBaseGrid) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn r#type(this: &JsBaseGrid) -> GridType;

    #[wasm_bindgen(method, js_name = isGridless)]
    pub fn is_gridless(this: &JsBaseGrid) -> bool;

    #[wasm_bindgen(method, js_name = isSquare)]
    pub fn is_square(this: &JsBaseGrid) -> bool;

    #[wasm_bindgen(method, js_name = isHexagonal)]
    pub fn is_hexagonal(this: &JsBaseGrid) -> bool;
}
