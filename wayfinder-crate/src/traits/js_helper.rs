use crate::traits::JsDeserialize;
use std::thread::LocalKey;
use wasm_bindgen::prelude::*;

pub trait JsHelper {
    fn get(&self, key: &str) -> JsValue;
    fn get_value<D: JsDeserialize>(&self, key: &str) -> D;
    fn set(&self, key: &str, value: JsValue) -> bool;
}

pub trait StaticJsHelper {
    fn get(&'static self, key: &str) -> JsValue;
    fn get_value<T>(&'static self, key: &str) -> T
    where
        T: JsDeserialize;
    fn set(&'static self, key: &str, value: JsValue) -> bool;
}

impl<T> JsHelper for T
where
    T: JsCast,
{
    fn get(&self, key: &str) -> JsValue {
        js_sys::Reflect::get(self.as_ref(), &JsValue::from_str(key)).unwrap()
    }

    fn get_value<D: JsDeserialize>(&self, key: &str) -> D {
        JsDeserialize::from_js(self.get(key))
    }

    fn set(&self, key: &str, value: JsValue) -> bool {
        js_sys::Reflect::set(self.as_ref(), &JsValue::from_str(key), &value).unwrap_or(false)
    }
}

impl StaticJsHelper for LocalKey<JsValue> {
    fn get(&'static self, key: &str) -> JsValue {
        self.with(JsValue::clone).get(key)
    }

    fn get_value<D: JsDeserialize>(&'static self, key: &str) -> D {
        JsDeserialize::from_js(self.get(key))
    }

    fn set(&'static self, key: &str, value: JsValue) -> bool {
        self.with(JsValue::clone).set(key, value)
    }
}
