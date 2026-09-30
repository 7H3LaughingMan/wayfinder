use crate::types::{foundry::documents::JsSceneDimensions, shapes::Rectangle};

#[derive(Clone, Copy, Debug)]
pub struct SceneDimensions {
    pub width: f64,
    pub height: f64,
    pub size: f64,
    pub rect: Rectangle,
    pub scene_x: f64,
    pub scene_y: f64,
    pub scene_width: f64,
    pub scene_height: f64,
    pub scene_rect: Rectangle,
    pub distance: f64,
    pub ditance_pixels: f64,
    pub ratio: f64,
    pub max_r: f64,
    pub rows: i32,
    pub columns: i32,
}

impl From<JsSceneDimensions> for SceneDimensions {
    fn from(value: JsSceneDimensions) -> Self {
        SceneDimensions {
            width: value.width(),
            height: value.height(),
            size: value.size(),
            rect: value.rect().into(),
            scene_x: value.scene_x(),
            scene_y: value.scene_y(),
            scene_width: value.scene_width(),
            scene_height: value.scene_height(),
            scene_rect: value.scene_rect().into(),
            distance: value.distance(),
            ditance_pixels: value.distance_pixels(),
            ratio: value.ratio(),
            max_r: value.max_r(),
            rows: value.rows(),
            columns: value.columns(),
        }
    }
}
