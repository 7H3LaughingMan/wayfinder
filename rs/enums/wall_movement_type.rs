#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub enum WallMovementType {
    None = 0,
    Normal = 20,
}

impl crate::traits::JsDeserialize for WallMovementType {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        let value = i32::from_js(data);

        match value {
            0 => WallMovementType::None,
            20 => WallMovementType::Normal,
            _ => panic!("Unknown Wall Movement Type - {value}"),
        }
    }
}
