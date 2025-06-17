use crate::traits::{JsDeserialize, JsHelper, JsSerialize};

#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct GridMeasurePathResult {
    pub distance: f64,
    pub cost: f64,
    pub spaces: i32,
    pub diagonals: i32,
    pub euclidean: f64,
}

impl JsDeserialize for GridMeasurePathResult {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        GridMeasurePathResult {
            distance: data.get_value("distance"),
            cost: data.get_value("cost"),
            spaces: data.get_value("spaces"),
            diagonals: data.get_value("diagonals"),
            euclidean: data.get_value("euclidean"),
        }
    }
}
