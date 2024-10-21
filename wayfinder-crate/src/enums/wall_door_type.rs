#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub enum WallDoorType {
    None = 0,
    Door = 1,
    Secret = 2,
}

impl crate::traits::JsDeserialize for WallDoorType {
    fn from_value(value: wasm_bindgen::JsValue) -> Self {
        let value = i32::from_value(value);

        match value {
            0 => WallDoorType::None,
            1 => WallDoorType::Door,
            2 => WallDoorType::Secret,
            _ => panic!("Unknown Wall Door Type - {value}"),
        }
    }
}
