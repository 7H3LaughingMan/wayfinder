use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;

use crate::{
    enums::Grid,
    exports::{Fog, Walls},
    traits::{AStar, JsDeserialize, JsDeserializeVector, JsSerialize},
    types::{
        GLTexture, GridMeasurePathResult, Point, Rectangle, TokenDocument, TokenFindMovementPathWaypoint,
        TokenMovementWaypoint,
    },
};

#[wasm_bindgen(typescript_custom_section)]
const TYPESCRIPT_CUSTOM_SECTION: &'static str = r#"
import { TokenFindMovementPathWaypoint, TokenMovementWaypoint } from "foundry-pf2e/foundry/client/_types.mjs";
import { Point, Rectangle } from "foundry-pf2e/foundry/common/_types.mjs";
import { GridOffset2D } from "foundry-pf2e/foundry/common/grid/_types.mjs";"#;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(
        typescript_type = "foundry.grid.SquareGrid | foundry.grid.HexagonalGrid | foundry.grid.GridlessGrid"
    )]
    pub type JsGrid;

    #[derive(Debug)]
    #[wasm_bindgen(typescript_type = "GridMeasurePathResult")]
    pub type JsGridMeasurePathResult;

    #[derive(Debug)]
    #[wasm_bindgen(typescript_type = "GridOffset2D")]
    pub type JsGridOffset2D;

    #[derive(Debug)]
    #[wasm_bindgen(typescript_type = "PIXI.GLTexture")]
    pub type JsGLTexture;

    #[derive(Debug)]
    #[wasm_bindgen(typescript_type = "Point")]
    pub type JsPoint;

    #[derive(Debug)]
    #[wasm_bindgen(typescript_type = "Rectangle")]
    pub type JsRectangle;

    #[derive(Debug)]
    #[wasm_bindgen(typescript_type = "foundry.documents.TokenDocument")]
    pub type JsTokenDocument;

    #[derive(Debug)]
    #[wasm_bindgen(typescript_type = "TokenFindMovementPathWaypoint")]
    pub type JsTokenFindMovementPathWaypoint;

    #[derive(Debug)]
    #[wasm_bindgen(typescript_type = "TokenMovementWaypoint")]
    pub type JsTokenMovementWaypoint;

    #[derive(Debug)]
    #[wasm_bindgen(typescript_type = "foundry.documents.WallDocument")]
    pub type JsWallDocument;
}

#[wasm_bindgen]
pub struct Wayfinder {
    bounds: Rectangle,
    fog: Option<Fog>,
    grid: Grid,
    walls: Walls,
}

#[wasm_bindgen]
impl Wayfinder {
    #[wasm_bindgen(constructor)]
    pub fn new(bounds: JsRectangle, grid: JsGrid, wall_documents: Vec<JsWallDocument>) -> Wayfinder {
        let bounds = Rectangle::from_js(bounds);
        let grid = Grid::from_js(grid);
        let walls = Walls::new(bounds, wall_documents);

        Wayfinder { bounds, fog: None, grid, walls }
    }

    #[wasm_bindgen(js_name = updateFog)]
    pub fn update_fog(
        &mut self,
        gl: WebGl2RenderingContext,
        gl_texture: JsGLTexture,
        bounds: JsRectangle,
        resolution: f64,
    ) {
        self.fog = Some(Fog::new(gl, GLTexture::from_js(gl_texture), Rectangle::from_js(bounds), resolution))
    }

    #[wasm_bindgen(js_name = isPointExplored)]
    pub fn is_point_explored(&mut self, point: JsPoint) -> bool {
        match &self.fog {
            Some(fog) => fog.is_point_explored(Point::from_js(point)),
            None => false,
        }
    }

    #[wasm_bindgen(js_name = addWall)]
    pub fn add_wall(&mut self, wall_document: JsWallDocument) {
        self.walls.add_wall(wall_document);
    }

    #[wasm_bindgen(js_name = deleteWall)]
    pub fn delete_wall(&mut self, wall_document: JsWallDocument) {
        self.walls.delete_wall(wall_document);
    }

    #[wasm_bindgen(js_name = updateWall)]
    pub fn update_wall(&mut self, wall_document: JsWallDocument) {
        self.walls.update_wall(wall_document);
    }

    #[wasm_bindgen(js_name = findMovementPath)]
    pub async fn find_movement_path(
        &mut self,
        token: JsTokenDocument,
        waypoints: Vec<JsTokenFindMovementPathWaypoint>,
        use_exploration: bool,
        grid_measure_path_result: JsGridMeasurePathResult,
    ) -> Vec<JsTokenMovementWaypoint> {
        let token = TokenDocument::from_js(token);
        let waypoints = TokenFindMovementPathWaypoint::from_js_vector(waypoints);
        let grid_measure_path_result = GridMeasurePathResult::from_js(grid_measure_path_result);

        let mut new_waypoints = Vec::<TokenMovementWaypoint>::new();
        let mut default_waypoint = &token.create_waypoint();
        for waypoint in waypoints {
            new_waypoints.push(waypoint.create_waypoint(default_waypoint));
            default_waypoint = new_waypoints.last().unwrap();
        }

        let path = match &self.grid {
            Grid::Gridless(gridless_grid) => gridless_grid.find_path(
                new_waypoints,
                &token,
                &self.bounds,
                &self.walls,
                if use_exploration { &self.fog } else { &None },
                &grid_measure_path_result,
            ),
            Grid::Square(square_grid) => square_grid.find_path(
                new_waypoints,
                &token,
                &self.bounds,
                &self.walls,
                if use_exploration { &self.fog } else { &None },
                &grid_measure_path_result,
            ),
            Grid::Hexagonal(hexagonal_grid) => hexagonal_grid.find_path(
                new_waypoints,
                &token,
                &self.bounds,
                &self.walls,
                if use_exploration { &self.fog } else { &None },
                &grid_measure_path_result,
            ),
        };

        path.iter().map(|waypoint| JsSerialize::to_value(waypoint).into()).collect()
    }
}
