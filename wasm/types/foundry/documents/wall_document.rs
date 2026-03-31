use crate::types::foundry::{WallDirection, WallDoorState, WallDoorType, WallMovementType, WallSenseType};
use js_sys::{ArrayTuple, Number, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(
        extends = Object,
        js_name = WallDocument,
        js_namespace = ["foundry", "documents"],
        typescript_type = "foundry.documents.WallDocument"
    )]
    pub type JsWallDocument;

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &JsWallDocument) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn c(this: &JsWallDocument) -> ArrayTuple<(Number, Number, Number, Number)>;

    #[wasm_bindgen(method, getter)]
    pub fn light(this: &JsWallDocument) -> WallSenseType;

    #[wasm_bindgen(method, getter)]
    pub fn r#move(this: &JsWallDocument) -> WallMovementType;

    #[wasm_bindgen(method, getter)]
    pub fn sight(this: &JsWallDocument) -> WallSenseType;

    #[wasm_bindgen(method, getter)]
    pub fn sound(this: &JsWallDocument) -> WallSenseType;

    #[wasm_bindgen(method, getter)]
    pub fn dir(this: &JsWallDocument) -> WallDirection;

    #[wasm_bindgen(method, getter)]
    pub fn door(this: &JsWallDocument) -> WallDoorType;

    #[wasm_bindgen(method, getter)]
    pub fn ds(this: &JsWallDocument) -> WallDoorState;
}
