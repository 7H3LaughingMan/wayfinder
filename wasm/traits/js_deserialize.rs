use wasm_bindgen::convert::TryFromJsValue;

pub trait JsDeserialize {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self;
}

pub trait JsDeserializeVector
where Self: Sized
{
    fn from_js_vector(data: Vec<impl wasm_bindgen::JsCast>) -> Vec<Self>;
}

impl<T: JsDeserialize> JsDeserializeVector for T {
    fn from_js_vector(data: Vec<impl wasm_bindgen::JsCast>) -> Vec<Self> {
        data.into_iter().map(|data| T::from_js(data)).collect()
    }
}

impl<T: JsDeserialize> JsDeserialize for Vec<T> {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        match js_sys::try_iter(data.as_ref()) {
            Ok(data) => match data {
                Some(data) => data
                    .filter_map(|v| match v {
                        Ok(v) => Some(T::from_js(v)),
                        Err(_) => None,
                    })
                    .collect(),
                None => Vec::new(),
            },
            Err(_) => Vec::new(),
        }
    }
}

impl<T: JsDeserialize + Copy + Default, const N: usize> JsDeserialize for [T; N] {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        Vec::<T>::from_js(data).try_into().unwrap_or([T::default(); N])
    }
}

impl<T: JsDeserialize> JsDeserialize for Option<T> {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        match data.as_ref().is_null_or_undefined() {
            true => None,
            false => Some(T::from_js(data)),
        }
    }
}

impl JsDeserialize for String {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        Self::try_from_js_value_ref(data.as_ref()).unwrap_or_default()
    }
}

impl<'a> JsDeserialize for &'a str {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        String::from_js(data).leak()
    }
}

impl JsDeserialize for bool {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        Self::try_from_js_value_ref(data.as_ref()).unwrap_or_default()
    }
}

impl JsDeserialize for char {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        Self::try_from_js_value_ref(data.as_ref()).unwrap_or_default()
    }
}

impl JsDeserialize for i8 {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        Self::try_from_js_value_ref(data.as_ref()).unwrap_or_default()
    }
}

impl JsDeserialize for u8 {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        Self::try_from_js_value_ref(data.as_ref()).unwrap_or_default()
    }
}

impl JsDeserialize for i16 {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        Self::try_from_js_value_ref(data.as_ref()).unwrap_or_default()
    }
}

impl JsDeserialize for u16 {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        Self::try_from_js_value_ref(data.as_ref()).unwrap_or_default()
    }
}

impl JsDeserialize for i32 {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        Self::try_from_js_value_ref(data.as_ref()).unwrap_or_default()
    }
}

impl JsDeserialize for u32 {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        Self::try_from_js_value_ref(data.as_ref()).unwrap_or_default()
    }
}

impl JsDeserialize for i64 {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        Self::try_from_js_value_ref(data.as_ref()).unwrap_or_default()
    }
}

impl JsDeserialize for u64 {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        Self::try_from_js_value_ref(data.as_ref()).unwrap_or_default()
    }
}

impl JsDeserialize for i128 {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        Self::try_from_js_value_ref(data.as_ref()).unwrap_or_default()
    }
}

impl JsDeserialize for u128 {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        Self::try_from_js_value_ref(data.as_ref()).unwrap_or_default()
    }
}

impl JsDeserialize for isize {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        Self::try_from_js_value_ref(data.as_ref()).unwrap_or_default()
    }
}

impl JsDeserialize for usize {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        Self::try_from_js_value_ref(data.as_ref()).unwrap_or_default()
    }
}

impl JsDeserialize for f32 {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        Self::try_from_js_value_ref(data.as_ref()).unwrap_or_default()
    }
}

impl JsDeserialize for f64 {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        Self::try_from_js_value(data.into()).unwrap_or_default()
    }
}
