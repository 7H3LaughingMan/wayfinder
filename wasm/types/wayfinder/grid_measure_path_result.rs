use crate::types::foundry::grid::JsGridMeasurePathResult;

pub struct GridMeasurePathResult {
    pub distance: f64,
    pub cost: f64,
    pub spaces: i32,
    pub diagonals: i32,
    pub euclidiean: f64,
}

impl From<JsGridMeasurePathResult> for GridMeasurePathResult {
    fn from(value: JsGridMeasurePathResult) -> Self {
        GridMeasurePathResult {
            distance: value.distance(),
            cost: value.cost(),
            spaces: value.spaces() as i32,
            diagonals: value.diagonals() as i32,
            euclidiean: value.euclidean(),
        }
    }
}
