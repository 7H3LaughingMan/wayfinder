use crate::types::helpers::JsObject;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(typescript_type = "foundry.TokenFindMovementPathJob")]
    pub type JsTokenFindMovementPathJob;
}

impl JsTokenFindMovementPathJob {
    pub fn new<P, C>(promise: P, cancel: C) -> JsTokenFindMovementPathJob
    where
        P: Future<Output = Result<JsValue, JsValue>> + 'static,
        C: Fn() -> () + 'static,
    {
        JsObject::new()
            .set_undefined("result")
            .set("promise", future_to_promise(promise))
            .set("cancel", Closure::<dyn Fn() -> ()>::new(cancel).into_js_value())
            .unchecked_into()
    }
}
