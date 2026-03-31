use wasm_bindgen::{JsCast, JsValue};

pub trait JsSerialize {
    fn to_js(self) -> JsValue;
}

impl<T: JsSerialize> JsSerialize for Vec<T> {
    fn to_js(self) -> JsValue {
        self.into_iter().map(|v| v.to_js()).collect::<js_sys::Array>().unchecked_into()
    }
}

impl<T: JsSerialize, const N: usize> JsSerialize for [T; N] {
    fn to_js(self) -> JsValue {
        self.into_iter().map(|v| v.to_js()).collect::<js_sys::Array>().unchecked_into()
    }
}

impl<T: JsSerialize> JsSerialize for Option<T> {
    fn to_js(self) -> JsValue {
        match self {
            Some(value) => value.to_js(),
            None => JsValue::undefined(),
        }
    }
}

impl JsSerialize for String {
    fn to_js(self) -> JsValue {
        JsValue::from(self)
    }
}

impl<'a> JsSerialize for &'a str {
    fn to_js(self) -> JsValue {
        JsValue::from(self)
    }
}

impl JsSerialize for bool {
    fn to_js(self) -> JsValue {
        JsValue::from(self)
    }
}

impl JsSerialize for char {
    fn to_js(self) -> JsValue {
        JsValue::from(self.to_string())
    }
}

impl JsSerialize for i8 {
    fn to_js(self) -> JsValue {
        JsValue::from(self)
    }
}

impl JsSerialize for u8 {
    fn to_js(self) -> JsValue {
        JsValue::from(self)
    }
}

impl JsSerialize for i16 {
    fn to_js(self) -> JsValue {
        JsValue::from(self)
    }
}

impl JsSerialize for u16 {
    fn to_js(self) -> JsValue {
        JsValue::from(self)
    }
}

impl JsSerialize for i32 {
    fn to_js(self) -> JsValue {
        JsValue::from(self)
    }
}

impl JsSerialize for u32 {
    fn to_js(self) -> JsValue {
        JsValue::from(self)
    }
}

impl JsSerialize for i64 {
    fn to_js(self) -> JsValue {
        JsValue::from(self)
    }
}

impl JsSerialize for u64 {
    fn to_js(self) -> JsValue {
        JsValue::from(self)
    }
}

impl JsSerialize for i128 {
    fn to_js(self) -> JsValue {
        JsValue::from(self)
    }
}

impl JsSerialize for u128 {
    fn to_js(self) -> JsValue {
        JsValue::from(self)
    }
}

impl JsSerialize for isize {
    fn to_js(self) -> JsValue {
        JsValue::from(self)
    }
}

impl JsSerialize for usize {
    fn to_js(self) -> JsValue {
        JsValue::from(self)
    }
}

impl JsSerialize for f32 {
    fn to_js(self) -> JsValue {
        JsValue::from(self)
    }
}

impl JsSerialize for f64 {
    fn to_js(self) -> JsValue {
        JsValue::from(self)
    }
}
