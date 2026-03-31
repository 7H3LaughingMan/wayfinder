use js_sys::{Array, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type JsObject;

    #[wasm_bindgen(method, indexing_getter)]
    fn getter(this: &JsObject, prop: &str) -> JsValue;

    #[wasm_bindgen(method, indexing_setter)]
    fn setter(this: &JsObject, prop: &str, val: JsValue);

    #[wasm_bindgen(method, indexing_deleter)]
    fn deleter(this: &JsObject, prop: &str);
}

impl JsObject {
    pub fn new() -> JsObject {
        Object::new().unchecked_into()
    }

    pub fn get<T: JsCast>(self, key: &str) -> T {
        self.getter(key).unchecked_into()
    }

    pub fn keys(self) -> Array<String> {
        Object::keys(&self).unchecked_into()
    }

    pub fn set(self, key: &str, value: impl Into<JsValue>) -> Self {
        let value = value.into();
        if !value.is_undefined() {
            self.setter(key, value);
        }
        self
    }

    pub fn set_undefined(self, key: &str) -> Self {
        self.setter(key, JsValue::undefined());
        self
    }
}
