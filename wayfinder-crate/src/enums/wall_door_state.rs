#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub enum WallDoorState {
    Closed = 0,
    Open = 1,
    Locked = 2,
}

impl crate::traits::JsDeserialize for WallDoorState {
    fn from_value(value: wasm_bindgen::JsValue) -> Self {
        let value = i32::from_value(value);

        match value {
            0 => WallDoorState::Closed,
            1 => WallDoorState::Open,
            2 => WallDoorState::Locked,
            _ => panic!("Unknown Wall Door State - {value}"),
        }
    }
}
