use crate::types::foundry::{EdgeDirection, EdgeSenseType, WallDoorState, WallDoorType, WallMovementType};
use js_sys::{ArrayTuple, JsString, Number, Object, Set};
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
    pub fn levels(this: &JsWallDocument) -> Set<JsString>;

    #[wasm_bindgen(method, getter)]
    pub fn light(this: &JsWallDocument) -> EdgeSenseType;

    #[wasm_bindgen(method, getter)]
    pub fn r#move(this: &JsWallDocument) -> WallMovementType;

    #[wasm_bindgen(method, getter)]
    pub fn sight(this: &JsWallDocument) -> EdgeSenseType;

    #[wasm_bindgen(method, getter)]
    pub fn sound(this: &JsWallDocument) -> EdgeSenseType;

    #[wasm_bindgen(method, getter)]
    pub fn dir(this: &JsWallDocument) -> EdgeDirection;

    #[wasm_bindgen(method, getter)]
    pub fn door(this: &JsWallDocument) -> WallDoorType;

    #[wasm_bindgen(method, getter)]
    pub fn ds(this: &JsWallDocument) -> WallDoorState;
}
