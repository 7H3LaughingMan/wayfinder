use crate::types::{
    Scene,
    foundry::{WallDirection, WallDoorState, WallDoorType, WallMovementType, WallSenseType},
    helpers::{JsArray, JsObject},
};
use geo::{Coord, CoordsIter};
use js_sys::{Array, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(
        extends = Object,
        js_name = Scene,
        js_namespace = ["foundry", "documents"],
        typescript_type = "foundry.documents.Scene"
    )]
    pub type JsScene;
}

impl From<Scene> for JsScene {
    fn from(value: Scene) -> Self {
        JsObject::new()
            .set("name", "Maze")
            .set(
                "background",
                JsObject::new()
                    .set("src", value.svg)
                    .set("anchorX", 0)
                    .set("anchorY", 0)
                    .set("offsetX", 0)
                    .set("offsetY", 0)
                    .set("fit", "fill")
                    .set("scaleX", 1)
                    .set("scaleY", 1)
                    .set("rotation", 0)
                    .set("tint", "#ffffff")
                    .set("alphaThreshold", 0),
            )
            .set("width", value.width)
            .set("height", value.height)
            .set("padding", 0)
            .set(
                "grid",
                JsObject::new()
                    .set("type", 1)
                    .set("size", 100)
                    .set("style", "solidLines")
                    .set("thickness", 1)
                    .set("color", "#000000")
                    .set("alpha", 0.2)
                    .set("distance", 5)
                    .set("units", "ft"),
            )
            .set("tokenVision", false)
            .set(
                "fog",
                JsObject::new()
                    .set("exploration", false)
                    .set("overlay", JsValue::null())
                    .set("colors", JsObject::new().set("explored", JsValue::null()).set("unexplored", JsValue::null())),
            )
            .set(
                "walls",
                value
                    .walls
                    .iter()
                    .map(|wall| {
                        JsObject::new()
                            .set("c", JsArray::new(wall.coords_iter().flat_map(|Coord { x, y }| [x, y])))
                            .set("light", WallSenseType::Normal)
                            .set("move", WallMovementType::Normal)
                            .set("sight", WallSenseType::Normal)
                            .set("sound", WallSenseType::Normal)
                            .set("dir", WallDirection::Both)
                            .set("door", WallDoorType::None)
                            .set("ds", WallDoorState::Closed)
                            .clone()
                    })
                    .collect::<Array>(),
            )
            .unchecked_into()
    }
}
