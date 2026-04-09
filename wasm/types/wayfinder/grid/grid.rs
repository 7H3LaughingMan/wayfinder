use crate::{
    CANVAS,
    exports::CancellationToken,
    traits::{AStar, BaseGrid},
    types::{
        foundry::{
            GridType,
            grid::{JsGridlessGrid, JsHexagonalGrid, JsSquareGrid},
        },
        wayfinder::{
            ElevatedPoint, FogManager, GridMeasurePathResult, GridOffset3D, RegionManager, TokenDocument,
            TokenMovementWaypoint, WallManager,
            grid::{GridlessGrid, HexagonalGrid, SquareGrid},
        },
    },
};
use geo::Rect;
use wasm_bindgen::JsCast;

#[derive(Clone, Debug)]
pub enum Grid {
    Gridless(GridlessGrid),
    Square(SquareGrid),
    Hexagonal(HexagonalGrid),
}

impl Grid {
    pub fn new() -> Self {
        let base_grid = CANVAS.grid().unwrap();

        match base_grid.r#type() {
            GridType::Gridless => {
                let gridless_grid = base_grid.unchecked_ref::<JsGridlessGrid>();
                Grid::Gridless(GridlessGrid { size: gridless_grid.size() as i32, distance: gridless_grid.distance() })
            }
            GridType::Square => {
                let square_grid = base_grid.unchecked_ref::<JsSquareGrid>();
                Grid::Square(SquareGrid { size: square_grid.size() as i32, distance: square_grid.distance() })
            }
            GridType::HexOddR | GridType::HexEvenR | GridType::HexOddQ | GridType::HexEvenQ => {
                let hexagonal_grid = base_grid.unchecked_ref::<JsHexagonalGrid>();
                Grid::Hexagonal(HexagonalGrid {
                    size: hexagonal_grid.size() as i32,
                    distance: hexagonal_grid.distance(),
                    size_x: hexagonal_grid.size_x(),
                    size_y: hexagonal_grid.size_y(),
                    columns: hexagonal_grid.columns(),
                    even: hexagonal_grid.even(),
                })
            }
        }
    }

    pub fn distance(&self) -> f64 {
        match self {
            Grid::Gridless(gridless_grid) => gridless_grid.distance,
            Grid::Square(square_grid) => square_grid.distance,
            Grid::Hexagonal(hexagonal_grid) => hexagonal_grid.distance,
        }
    }

    pub fn get_center_point(&self, offset: GridOffset3D) -> ElevatedPoint {
        match self {
            Grid::Gridless(gridless_grid) => gridless_grid.get_offset_center_point(offset),
            Grid::Square(square_grid) => square_grid.get_offset_center_point(offset),
            Grid::Hexagonal(hexagonal_grid) => hexagonal_grid.get_offset_center_point(offset),
        }
    }

    pub fn get_top_left_point(&self, offset: GridOffset3D) -> ElevatedPoint {
        match self {
            Grid::Gridless(gridless_grid) => gridless_grid.get_offset_top_left_point(offset),
            Grid::Square(square_grid) => square_grid.get_offset_top_left_point(offset),
            Grid::Hexagonal(hexagonal_grid) => hexagonal_grid.get_offset_top_left_point(offset),
        }
    }

    pub fn size(&self) -> f64 {
        match self {
            Grid::Gridless(gridless_grid) => gridless_grid.size as f64,
            Grid::Square(square_grid) => square_grid.size as f64,
            Grid::Hexagonal(hexagonal_grid) => hexagonal_grid.size as f64,
        }
    }

    pub fn size_x(&self) -> f64 {
        match self {
            Grid::Gridless(gridless_grid) => gridless_grid.size as f64,
            Grid::Square(square_grid) => square_grid.size as f64,
            Grid::Hexagonal(hexagonal_grid) => hexagonal_grid.size_x,
        }
    }

    pub fn size_y(&self) -> f64 {
        match self {
            Grid::Gridless(gridless_grid) => gridless_grid.size as f64,
            Grid::Square(square_grid) => square_grid.size as f64,
            Grid::Hexagonal(hexagonal_grid) => hexagonal_grid.size_y,
        }
    }

    pub fn find_path(
        &self,
        cancellation_token: &CancellationToken,
        waypoints: Vec<TokenMovementWaypoint>,
        token: &TokenDocument,
        scene_rect: &Rect,
        fog_manager: Option<&FogManager>,
        region_manager: &RegionManager,
        wall_manager: &WallManager,
        grid_measure_path_result: &GridMeasurePathResult,
    ) -> Vec<TokenMovementWaypoint> {
        match self {
            Grid::Gridless(gridless_grid) => gridless_grid.find_path(
                cancellation_token,
                waypoints,
                token,
                scene_rect,
                fog_manager,
                region_manager,
                wall_manager,
                grid_measure_path_result,
            ),
            Grid::Square(square_grid) => square_grid.find_path(
                cancellation_token,
                waypoints,
                token,
                scene_rect,
                fog_manager,
                region_manager,
                wall_manager,
                grid_measure_path_result,
            ),
            Grid::Hexagonal(hexagonal_grid) => hexagonal_grid.find_path(
                cancellation_token,
                waypoints,
                token,
                scene_rect,
                fog_manager,
                region_manager,
                wall_manager,
                grid_measure_path_result,
            ),
        }
    }
}
