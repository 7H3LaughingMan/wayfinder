use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct CancellationToken {
    token: Arc<AtomicBool>,
}

#[wasm_bindgen]
impl CancellationToken {
    #[wasm_bindgen(constructor)]
    pub fn new() -> CancellationToken {
        CancellationToken { token: Arc::new(AtomicBool::new(false)) }
    }

    #[wasm_bindgen]
    pub fn cancel(&self) {
        self.token.store(true, Ordering::Relaxed);
    }

    #[wasm_bindgen]
    pub fn status(&self) -> bool {
        self.token.load(Ordering::Relaxed)
    }
}
