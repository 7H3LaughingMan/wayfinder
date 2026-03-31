pub struct JsNullable<T>(Option<T>);

impl<T> JsNullable<T> {
    pub fn new(value: T) -> Self {
        JsNullable(Some(value))
    }

    pub fn null() -> Self {
        JsNullable(None)
    }
}

impl<T> std::ops::Deref for JsNullable<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for JsNullable<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<Option<T>> for JsNullable<T> {
    fn from(value: Option<T>) -> Self {
        JsNullable(value)
    }
}

impl<T> From<JsNullable<T>> for wasm_bindgen::JsValue
where wasm_bindgen::JsValue: From<T>
{
    fn from(value: JsNullable<T>) -> Self {
        match value.0 {
            Some(value) => value.into(),
            None => wasm_bindgen::JsValue::null(),
        }
    }
}
