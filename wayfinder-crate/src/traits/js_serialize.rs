use wasm_bindgen::prelude::*;

pub trait JsSerialize
    where Self: Sized
{
    fn to_value(value: Self) -> JsValue;
}

impl<V: JsSerialize + Clone> JsSerialize for Vec<V> {
    fn to_value(value: Self) -> JsValue {
        let array = js_sys::Array::new_with_length(value.len() as u32);

        for (i, v) in value.iter().enumerate() {
            array.set(i as u32, JsSerialize::to_value(v.clone()));
        }

        return array.into();
    }
}

impl JsSerialize for String {
    fn to_value(value: Self) -> JsValue {
        JsValue::from_str(&value)
    }
}

impl JsSerialize for bool {
    fn to_value(value: Self) -> JsValue {
        JsValue::from_bool(value)
    }
}

impl JsSerialize for f64 {
    fn to_value(value: Self) -> JsValue {
        JsValue::from_f64(value)
    }
}

impl JsSerialize for f32 {
    fn to_value(value: Self) -> JsValue {
        JsValue::from_f64(value as f64)
    }
}

impl JsSerialize for i8 {
    fn to_value(value: Self) -> JsValue {
        JsValue::from_f64(value as f64)
    }
}

impl JsSerialize for u8 {
    fn to_value(value: Self) -> JsValue {
        JsValue::from_f64(value as f64)
    }
}

impl JsSerialize for i16 {
    fn to_value(value: Self) -> JsValue {
        JsValue::from_f64(value as f64)
    }
}

impl JsSerialize for u16 {
    fn to_value(value: Self) -> JsValue {
        JsValue::from_f64(value as f64)
    }
}

impl JsSerialize for i32 {
    fn to_value(value: Self) -> JsValue {
        JsValue::from_f64(value as f64)
    }
}

impl JsSerialize for u32 {
    fn to_value(value: Self) -> JsValue {
        JsValue::from_f64(value as f64)
    }
}
