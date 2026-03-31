use js_sys::Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(extends = Array)]
    pub type JsArray;
}

impl JsArray {
    pub fn new<T>(values: impl IntoIterator<Item = T>) -> Self
    where T: Into<JsValue> {
        values.into_iter().map(|value| value.into()).collect::<Array>().unchecked_into()
    }
}
