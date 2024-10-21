use wasm_bindgen::prelude::*;

pub trait JsHelper {
    fn get(&self, key: &str) -> JsValue;
    fn set(&self, key: &str, value: JsValue) -> bool;
}

impl JsHelper for JsValue {
    fn get(&self, key: &str) -> JsValue {
        js_sys::Reflect::get(self, &JsValue::from_str(key)).unwrap()
    }

    fn set(&self, key: &str, value: JsValue) -> bool {
        js_sys::Reflect::set(self, &JsValue::from_str(key), &value).unwrap_or(false)
    }
}
