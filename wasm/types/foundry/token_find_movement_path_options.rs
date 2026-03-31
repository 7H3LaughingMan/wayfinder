use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(typescript_type = "foundry.TokenFindMovementPathOptions")]
    pub type JsTokenFindMovementPathOptions;

    #[wasm_bindgen(method, getter)]
    pub fn preview(this: &JsTokenFindMovementPathOptions) -> Option<bool>;

    #[wasm_bindgen(method, getter = ignoreWalls)]
    pub fn ignore_walls(this: &JsTokenFindMovementPathOptions) -> Option<bool>;

    #[wasm_bindgen(method, getter = ignoreCost)]
    pub fn ignore_cost(this: &JsTokenFindMovementPathOptions) -> Option<bool>;

    #[wasm_bindgen(method, getter)]
    pub fn history(this: &JsTokenFindMovementPathOptions) -> Option<JsValue>;

    #[wasm_bindgen(method, getter)]
    pub fn delay(this: &JsTokenFindMovementPathOptions) -> Option<f64>;
}
