#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub enum RegionVisibility {
    Layer = 0,
    Gamemaster = 1,
    Always = 2,
}

impl crate::traits::JsDeserialize for RegionVisibility {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        let value = u32::from_js(data);

        match value {
            0 => RegionVisibility::Layer,
            1 => RegionVisibility::Gamemaster,
            2 => RegionVisibility::Always,
            _ => panic!("Unknown Region Visibility - {value}"),
        }
    }
}
