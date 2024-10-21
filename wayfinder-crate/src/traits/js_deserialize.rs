use wasm_bindgen::prelude::*;

pub trait JsDeserialize
    where Self: Sized
{
    fn from_value(value: JsValue) -> Self;
}

impl<V: JsDeserialize> JsDeserialize for Vec<V> {
    fn from_value(value: JsValue) -> Self {
        let mut vector = Vec::new();
        let iterator = js_sys::try_iter(&value).unwrap().unwrap();

        for x in iterator {
            vector.push(JsDeserialize::from_value(x.unwrap()));
        }

        return vector;
    }
}

impl JsDeserialize for String {
    fn from_value(value: JsValue) -> Self {
        value.as_string().unwrap()
    }
}

impl JsDeserialize for bool {
    fn from_value(value: JsValue) -> Self {
        value.as_bool().unwrap()
    }
}

impl JsDeserialize for f64 {
    fn from_value(value: JsValue) -> Self {
        value.as_f64().unwrap()
    }
}

impl JsDeserialize for f32 {
    fn from_value(value: JsValue) -> Self {
        value.as_f64().unwrap() as Self
    }
}

impl JsDeserialize for i8 {
    fn from_value(value: JsValue) -> Self {
        value.as_f64().unwrap() as Self
    }
}

impl JsDeserialize for u8 {
    fn from_value(value: JsValue) -> Self {
        value.as_f64().unwrap() as Self
    }
}

impl JsDeserialize for i16 {
    fn from_value(value: JsValue) -> Self {
        value.as_f64().unwrap() as Self
    }
}

impl JsDeserialize for u16 {
    fn from_value(value: JsValue) -> Self {
        value.as_f64().unwrap() as Self
    }
}

impl JsDeserialize for i32 {
    fn from_value(value: JsValue) -> Self {
        value.as_f64().unwrap() as Self
    }
}

impl JsDeserialize for u32 {
    fn from_value(value: JsValue) -> Self {
        value.as_f64().unwrap() as Self
    }
}
