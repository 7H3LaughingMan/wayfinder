use crate::traits::JsDeserialize;
use std::thread::LocalKey;
use wasm_bindgen::prelude::*;

pub trait JsHelper {
    fn get(&self, key: &str) -> JsValue;
    fn get_value<D: JsDeserialize>(&self, key: &str) -> D;
    fn has(&self, key: &str) -> bool;
    fn set(&self, key: &str, value: JsValue) -> bool;
}

impl<T> JsHelper for T
where T: JsCast
{
    fn get(&self, key: &str) -> JsValue {
        js_sys::Reflect::get(self.as_ref(), &JsValue::from_str(key)).unwrap()
    }

    fn get_value<D: JsDeserialize>(&self, key: &str) -> D {
        JsDeserialize::from_js(self.get(key))
    }

    fn has(&self, key: &str) -> bool {
        js_sys::Reflect::has(self.as_ref(), &JsValue::from_str(key)).unwrap()
    }

    fn set(&self, key: &str, value: JsValue) -> bool {
        js_sys::Reflect::set(self.as_ref(), &JsValue::from_str(key), &value).unwrap_or(false)
    }
}
