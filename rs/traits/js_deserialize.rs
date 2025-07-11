use std::fmt::Debug;
use wasm_bindgen::JsCast;

pub trait JsDeserialize {
    fn from_js(data: impl JsCast) -> Self;
}

pub trait JsDeserializeVector
where Self: Sized
{
    fn from_js_vector(data: Vec<impl JsCast>) -> Vec<Self>;
}

pub trait JsDeserializeOption
where Self: Sized
{
    fn from_js_option(data: Option<impl JsCast>) -> Option<Self>;
}

impl<T: JsDeserialize + Debug> JsDeserializeVector for T {
    fn from_js_vector(data: Vec<impl JsCast>) -> Vec<Self> {
        data.into_iter().map(|v| JsDeserialize::from_js(v)).collect()
    }
}

impl<T: JsDeserialize + Debug> JsDeserializeOption for T {
    fn from_js_option(data: Option<impl JsCast>) -> Option<Self> {
        if let Some(value) = data {
            Some(T::from_js(value))
        } else {
            None
        }
    }
}

impl<T: JsDeserialize + Debug, const L: usize> JsDeserialize for [T; L] {
    fn from_js(data: impl JsCast) -> Self {
        let iterator = js_sys::try_iter(data.as_ref()).unwrap().unwrap();
        let vector: Vec<T> = iterator.map(|v| JsDeserialize::from_js(v.unwrap())).collect();
        vector.try_into().unwrap()
    }
}

impl<T: JsDeserialize + Debug> JsDeserialize for Option<T> {
    fn from_js(data: impl JsCast) -> Self {
        if data.as_ref().is_undefined() {
            None
        } else {
            Some(T::from_js(data))
        }
    }
}

impl JsDeserialize for String {
    fn from_js(data: impl JsCast) -> Self {
        data.as_ref().as_string().unwrap()
    }
}

impl JsDeserialize for bool {
    fn from_js(data: impl JsCast) -> Self {
        data.as_ref().as_bool().unwrap()
    }
}

impl JsDeserialize for f64 {
    fn from_js(data: impl JsCast) -> Self {
        data.as_ref().as_f64().unwrap()
    }
}

impl JsDeserialize for f32 {
    fn from_js(data: impl JsCast) -> Self {
        data.as_ref().as_f64().unwrap() as Self
    }
}

impl JsDeserialize for i8 {
    fn from_js(data: impl JsCast) -> Self {
        data.as_ref().as_f64().unwrap() as Self
    }
}

impl JsDeserialize for i16 {
    fn from_js(data: impl JsCast) -> Self {
        data.as_ref().as_f64().unwrap() as Self
    }
}

impl JsDeserialize for i32 {
    fn from_js(data: impl JsCast) -> Self {
        data.as_ref().as_f64().unwrap() as Self
    }
}

impl JsDeserialize for u8 {
    fn from_js(data: impl JsCast) -> Self {
        data.as_ref().as_f64().unwrap() as Self
    }
}

impl JsDeserialize for u16 {
    fn from_js(data: impl JsCast) -> Self {
        data.as_ref().as_f64().unwrap() as Self
    }
}

impl JsDeserialize for u32 {
    fn from_js(data: impl JsCast) -> Self {
        data.as_ref().as_f64().unwrap() as Self
    }
}
