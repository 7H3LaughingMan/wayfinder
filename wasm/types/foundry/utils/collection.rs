use js_sys::{Array, Iterable, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(
        extends = Object,
        js_name = Collection,
        js_namespace = ["foundry", "utils"],
        typescript_type = "foundry.utils.Collection"
    )]
    pub type JsCollection<V = JsValue>;

    #[wasm_bindgen(method, getter)]
    pub fn contents<V>(this: &JsCollection<V>) -> Array<V>;
}

impl<V> Iterable for JsCollection<V> {
    type Item = V;
}
