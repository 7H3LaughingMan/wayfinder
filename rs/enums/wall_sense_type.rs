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
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        let value = u32::from_js(data);

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
