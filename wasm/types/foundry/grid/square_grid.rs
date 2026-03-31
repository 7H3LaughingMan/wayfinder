use crate::types::foundry::{GridDiagonalRule, grid::JsBaseGrid};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(
        extends = JsBaseGrid,
        extends = Object,
        js_name = SquareGrid,
        js_namespace = ["foundry", "grid"],
        typescript_type = "foundry.grid.SquareGrid"
    )]
    pub type JsSquareGrid;

    #[wasm_bindgen(method, getter)]
    pub fn diagonals(this: &JsSquareGrid) -> GridDiagonalRule;
}
