use crate::types::foundry::{GridDiagonalRule, grid::JsBaseGrid};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(
        extends = JsBaseGrid,
        extends = Object,
        js_name = HexagonalGrid,
        js_namespace = ["foundry", "grid"],
        typescript_type = "foundry.grid.HexagonalGrid"
    )]
    pub type JsHexagonalGrid;

    #[wasm_bindgen(method, getter)]
    pub fn columns(this: &JsHexagonalGrid) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn even(this: &JsHexagonalGrid) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn diagonals(this: &JsHexagonalGrid) -> GridDiagonalRule;
}
