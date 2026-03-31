use crate::types::foundry::grid::JsBaseGrid;
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(
        extends = JsBaseGrid,
        extends = Object,
        js_name = GridlessGrid,
        js_namespace = ["foundry", "grid"],
        typescript_type = "foundry.grid.GridlessGrid"
    )]
    pub type JsGridlessGrid;
}
