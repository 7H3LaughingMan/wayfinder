use crate::traits::{JsDeserialize, JsSerialize};
use wasm_bindgen::prelude::*;

pub trait JsHelper {
    fn get(&self, key: &str) -> JsValue;
    fn get_u32(&self, key: u32) -> JsValue;
    fn get_property<D: JsDeserialize>(&self, key: &str) -> D;
    fn get_property_u32<D: JsDeserialize>(&self, key: u32) -> D;
    fn get_value<D: JsCast>(&self, key: &str) -> D;
    fn get_value_u32<D: JsCast>(&self, key: u32) -> D;
    fn has(&self, key: &str) -> bool;
    fn set(&self, key: &str, value: JsValue) -> bool;
    fn set_property<S: JsSerialize>(&self, key: &str, value: S) -> &Self;
    fn set_value<S: JsCast>(&self, key: &str, value: S) -> &Self;
}

impl<T: JsCast> JsHelper for T {
    fn get(&self, key: &str) -> JsValue {
        js_sys::Reflect::get(self.as_ref(), &JsValue::from(key)).unwrap_or_default()
    }

    fn get_u32(&self, key: u32) -> JsValue {
        js_sys::Reflect::get(self.as_ref(), &JsValue::from(key)).unwrap_or_default()
    }

    fn get_property<D: JsDeserialize>(&self, key: &str) -> D {
        D::from_js(self.get(key))
    }

    fn get_property_u32<D: JsDeserialize>(&self, key: u32) -> D {
        D::from_js(self.get_u32(key))
    }

    fn get_value<D: JsCast>(&self, key: &str) -> D {
        self.get(key).unchecked_into()
    }

    fn get_value_u32<D: JsCast>(&self, key: u32) -> D {
        self.get_u32(key).unchecked_into()
    }

    fn has(&self, key: &str) -> bool {
        js_sys::Reflect::has(self.as_ref(), &JsValue::from(key)).unwrap_or_default()
    }

    fn set(&self, key: &str, value: JsValue) -> bool {
        js_sys::Reflect::set(self.as_ref(), &JsValue::from(key), &value).unwrap_or_default()
    }

    fn set_property<S: JsSerialize>(&self, key: &str, value: S) -> &Self {
        self.set(key, value.to_js());
        &self
    }

    fn set_value<S: JsCast>(&self, key: &str, value: S) -> &Self {
        self.set(key, value.into());
        &self
    }
}
