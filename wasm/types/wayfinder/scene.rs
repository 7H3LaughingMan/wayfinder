use crate::types::{
    foundry::{EdgeDirection, EdgeSenseType, WallDoorState, WallDoorType, WallMovementType},
    helpers::JsObject,
};
use base64::{Engine, prelude::BASE64_STANDARD};
use js_sys::{Array, ArrayTuple, Number};
use knossos::maze::{Cell, Eller, OrthogonalMazeBuilder};
use svg::{
    Document,
    node::element::{Path, path::Data},
};
use wasm_bindgen::JsValue;

const SCENE_SCALE: f64 = 100.0;

#[derive(Clone, Debug)]
pub struct Scene {
    pub width: f64,
    pub height: f64,
    pub walls: Vec<geo::Line>,
    pub svg: String,
}

impl Scene {
    pub fn new(width: usize, height: usize) -> Self {
        let maze = OrthogonalMazeBuilder::new().height(height).width(width).algorithm(Box::new(Eller)).build();

        let mut walls = Vec::<geo::Line>::new();

        for y in 0..maze.height() {
            for x in 0..maze.width() {
                if y != 0 && !maze.is_carved((x, y), Cell::NORTH) {
                    walls.push(geo::Line::from([
                        ((x as f64) * SCENE_SCALE, (y as f64) * SCENE_SCALE),
                        (((x + 1) as f64) * SCENE_SCALE, (y as f64) * SCENE_SCALE),
                    ]));
                }

                if x != 0 && !maze.is_carved((x, y), Cell::WEST) {
                    walls.push(geo::Line::from([
                        ((x as f64) * SCENE_SCALE, (y as f64) * SCENE_SCALE),
                        ((x as f64) * SCENE_SCALE, ((y + 1) as f64) * SCENE_SCALE),
                    ]));
                }
            }
        }

        let width = (maze.width() as f64) * SCENE_SCALE;
        let height = (maze.height() as f64) * SCENE_SCALE;
        let svg = Document::new()
            .set("height", height)
            .set("width", width)
            .add(
                Path::new().set("fill", "#fff").set("stroke", "#000").set("stroke-width", 10.0).set(
                    "d",
                    Data::new()
                        .move_to((0, 0))
                        .horizontal_line_by(width)
                        .vertical_line_by(height)
                        .horizontal_line_to(0)
                        .close(),
                ),
            )
            .add(
                Path::new().set("stroke", "#000").set("stroke-linecap", "square").set("stroke-width", 5.0).set(
                    "d",
                    walls
                        .iter()
                        .fold(Data::new(), |acc, wall| {
                            let start = wall.start.x_y();
                            let move_by = (wall.end - wall.start).x_y();

                            if move_by.0 != 0.0 {
                                acc.move_to(start).horizontal_line_by(move_by.0)
                            } else {
                                acc.move_to(start).vertical_line_by(move_by.1)
                            }
                        })
                        .close(),
                ),
            );

        Scene {
            width,
            height,
            walls,
            svg: format!("data:image/svg+xml;base64,{}", BASE64_STANDARD.encode(svg.to_string())),
        }
    }

    pub fn to_object(&self) -> JsObject {
        JsObject::new()
            .set("name", "Maze")
            .set(
                "background",
                JsObject::new()
                    .set("src", self.svg.clone())
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
            .set("width", self.width)
            .set("height", self.height)
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
                self.walls
                    .iter()
                    .map(|wall| {
                        JsObject::new()
                            .set(
                                "c",
                                ArrayTuple::new4(
                                    &Number::from(wall.start.x),
                                    &Number::from(wall.start.y),
                                    &Number::from(wall.end.x),
                                    &Number::from(wall.end.y),
                                ),
                            )
                            .set("light", EdgeSenseType::Normal)
                            .set("move", WallMovementType::Normal)
                            .set("sight", EdgeSenseType::Normal)
                            .set("sound", EdgeSenseType::Normal)
                            .set("dir", EdgeDirection::Both)
                            .set("door", WallDoorType::None)
                            .set("ds", WallDoorState::Closed)
                            .clone()
                    })
                    .collect::<Array>(),
            )
    }
}
