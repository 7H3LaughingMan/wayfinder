use crate::{
    enums::RegionVisibility,
    traits::{JsDeserialize, JsHelper},
};

#[derive(Clone)]
#[derive(Debug)]
pub struct RegionElevation {
    pub bottom: f64,
    pub top: f64,
}

impl JsDeserialize for RegionElevation {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        let bottom_value = data.get("bottom");
        let top_value = data.get("top");

        RegionElevation {
            bottom: if bottom_value.is_null() { f64::NEG_INFINITY } else { bottom_value.as_f64().unwrap() },
            top: if top_value.is_null() { f64::INFINITY } else { top_value.as_f64().unwrap() },
        }
    }
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Region {
    pub id: String,
    pub name: String,
    pub color: String,
    pub shapes: Vec<f64>,
    pub elevation: RegionElevation,
    pub behaviors: Vec<f64>,
    pub visibility: RegionVisibility,
    pub locked: bool,
}

impl JsDeserialize for Region {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        Region {
            id: data.get_value("_id"),
            name: data.get_value("name"),
            color: data.get_value("color"),
            shapes: data.get_value("shapes"),
            elevation: data.get_value("elevation"),
            behaviors: data.get_value("behaviors"),
            visibility: data.get_value("visibility"),
            locked: data.get_value("locked"),
        }
    }
}
