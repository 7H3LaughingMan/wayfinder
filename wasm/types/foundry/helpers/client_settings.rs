use js_sys::Object;
use wasm_bindgen::{convert::TryFromJsValue, prelude::*};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = Object,
        js_name = ClientSettings,
        js_namespace = ["foundry", "helpers"],
        typescript_type = "foundry.helpers.ClientSettings"
    )]
    pub type JsClientSettings;

    #[wasm_bindgen(method)]
    pub fn get(this: &JsClientSettings, namespace: &str, key: &str) -> JsValue;
}

impl JsClientSettings {
    pub fn get_setting<T: TryFromJsValue>(&self, namespace: &str, key: &str) -> T {
        T::try_from_js_value(self.get(namespace, key)).unwrap()
    }
}
