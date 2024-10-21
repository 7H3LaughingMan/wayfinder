#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub enum WallSenseType {
    None = 0,
    Limited = 10,
    Normal = 20,
    Proximity = 30,
    Distance = 40,
}

impl crate::traits::JsDeserialize for WallSenseType {
    fn from_value(value: wasm_bindgen::JsValue) -> Self {
        let value = i32::from_value(value);

        match value {
            0 => WallSenseType::None,
            10 => WallSenseType::Limited,
            20 => WallSenseType::Normal,
            30 => WallSenseType::Proximity,
            40 => WallSenseType::Distance,
            _ => panic!("Unknown Wall Sense Type - {value}"),
        }
    }
}
