use wasm_bindgen::prelude::*;

#[wasm_bindgen(skip_typescript)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub enum MSAA_QUALITY {
    NONE = 0,
    LOW = 2,
    MEDIUM = 4,
    HIGH = 8,
}
