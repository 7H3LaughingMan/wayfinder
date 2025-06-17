use std::{collections::HashMap, ops::RangeInclusive};

use crate::{
    enums::TokenShapeType,
    exports::Walls,
    nodes::GridlessNode,
    traits::{AStar, BaseGrid, JsDeserialize, JsHelper},
    types::{
        self, ElevatedPoint, GridMeasurePathResult, GridOffset2D, GridOffset3D, Point, Rectangle, TokenDocument,
        TokenMovementWaypoint, TokenSquareShapeData,
    },
};
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub struct GridlessGrid {
    pub size: i32,
    pub distance: f64,
}

impl BaseGrid<GridlessNode, TokenSquareShapeData> for GridlessGrid {
    fn convert_node_to_offset(&self, GridlessNode { i, j, k }: GridlessNode) -> GridOffset3D {
        GridOffset3D { i, j, k }
    }

    fn convert_offset_to_node(&self, GridOffset3D { i, j, k }: GridOffset3D) -> GridlessNode {
        GridlessNode { i, j, k }
    }

    fn get_adjacent_nodes(
        &self,
        _node: &GridlessNode,
        _token_shape: &TokenSquareShapeData,
        _elevation_range: &RangeInclusive<i32>,
        _bounds: &Rectangle,
        _walls: &Walls,
        _explored: &Option<HashMap<GridOffset2D, bool>>,
    ) -> Vec<(GridlessNode, u32)> {
        Vec::new()
    }

    fn get_node(
        &self,
        ElevatedPoint { x, y, elevation }: ElevatedPoint,
        _token_shape: &TokenSquareShapeData,
    ) -> GridlessNode {
        GridlessNode {
            i: y.floor() as i32,
            j: x.floor() as i32,
            k: ((elevation / self.distance * self.size as f64) + 1e-8).floor() as i32,
        }
    }

    fn get_node_center_point(&self, GridlessNode { i, j, k }: &GridlessNode) -> ElevatedPoint {
        ElevatedPoint { x: *j as f64, y: *i as f64, elevation: (*k as f64 / self.size as f64) * self.distance }
    }

    fn get_node_top_left_point(&self, GridlessNode { i, j, k }: &GridlessNode) -> ElevatedPoint {
        ElevatedPoint { x: *j as f64, y: *i as f64, elevation: (*k as f64 / self.size as f64) * self.distance }
    }

    fn get_occupied_grid_space_offsets(
        &self,
        _offset: GridOffset3D,
        _token_shape: &TokenSquareShapeData,
    ) -> Vec<GridOffset3D> {
        Vec::new()
    }

    fn get_offset(
        &self,
        ElevatedPoint { x, y, elevation }: ElevatedPoint,
        _token_shape: &TokenSquareShapeData,
    ) -> GridOffset3D {
        GridOffset3D {
            i: y.floor() as i32,
            j: x.floor() as i32,
            k: ((elevation / self.distance * self.size as f64) + 1e-8).floor() as i32,
        }
    }

    fn get_offset_center_point(&self, GridOffset3D { i, j, k }: GridOffset3D) -> ElevatedPoint {
        ElevatedPoint { x: j as f64, y: i as f64, elevation: (k as f64 / self.size as f64) * self.distance }
    }

    fn get_offset_top_left_point(&self, GridOffset3D { i, j, k }: GridOffset3D) -> ElevatedPoint {
        ElevatedPoint { x: j as f64, y: i as f64, elevation: (k as f64 / self.size as f64) * self.distance }
    }

    fn get_token_center_point(
        &self,
        ElevatedPoint { x, y, elevation }: ElevatedPoint,
        TokenSquareShapeData { offsets: _, points: _, center, anchor: _, width: _, height: _ }: &TokenSquareShapeData,
    ) -> ElevatedPoint {
        ElevatedPoint { x: x + (center.x * self.size as f64), y: y + (center.y * self.size as f64), elevation }
    }

    fn get_token_shape(&self, width: f64, height: f64, _shape: TokenShapeType) -> TokenSquareShapeData {
        let width = (width * 2.0).round() / 2.0;
        let height = (height * 2.0).round() / 2.0;

        TokenSquareShapeData {
            offsets: vec![],
            points: vec![
                Point { x: 0.0, y: 0.0 },
                Point { x: width, y: 0.0 },
                Point { x: width, y: height },
                Point { x: 0.0, y: height },
            ],
            center: Point { x: width / 2.0, y: height / 2.0 },
            anchor: Point { x: 0.0, y: 0.0 },
            width,
            height,
        }
    }
}

impl AStar for GridlessGrid {
    fn find_path(
        &self,
        _waypoints: Vec<TokenMovementWaypoint>,
        _token: &TokenDocument,
        _bounds: &Rectangle,
        _walls: &Walls,
        _explored: &Option<HashMap<GridOffset2D, bool>>,
        _grid_measure_path_result: &GridMeasurePathResult,
    ) -> Vec<TokenMovementWaypoint> {
        Vec::new()
    }
}
