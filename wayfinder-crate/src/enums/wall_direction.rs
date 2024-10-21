#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub enum WallDirection {
    Both = 0,
    Left = 1,
    Right = 2,
}

impl crate::traits::JsDeserialize for WallDirection {
    fn from_value(value: wasm_bindgen::JsValue) -> Self {
        let value = i32::from_value(value);

        match value {
            0 => WallDirection::Both,
            1 => WallDirection::Left,
            2 => WallDirection::Right,
            _ => panic!("Unknown Wall Direction - {value}"),
        }
    }
}