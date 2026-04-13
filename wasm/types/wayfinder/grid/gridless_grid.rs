use crate::{
    traits::{AStar, BaseGrid},
    types::{
        foundry::TokenShapeType,
        wayfinder::{
            ElevatedPoint, FogManager, GridMeasurePathResult, GridOffset3D, Point, RegionManager, TokenDocument,
            TokenMovementWaypoint, TokenSquareShape, WallManager, node::GridlessNode,
        },
    },
};
use geo::Rect;
use rust_decimal::{Decimal, dec};

#[derive(Clone, Debug)]
pub struct GridlessGrid {
    pub size: i32,
    pub distance: f64,
}

impl BaseGrid<GridlessNode, TokenSquareShape> for GridlessGrid {
    fn calculate_cost(
        &self,
        _from: GridlessNode,
        _to: GridlessNode,
        _token_shape: &TokenSquareShape,
        _level: &str,
        _fog_manager: Option<&FogManager>,
        _region_manager: &RegionManager,
        _wall_manager: &WallManager,
    ) -> Option<(GridlessNode, Decimal)> {
        None
    }

    fn convert_node_to_offset(&self, GridlessNode { i, j, k }: GridlessNode) -> GridOffset3D {
        GridOffset3D { i, j, k }
    }

    fn convert_offset_to_node(&self, GridOffset3D { i, j, k }: GridOffset3D) -> GridlessNode {
        GridlessNode { i, j, k }
    }

    fn get_adjacent_nodes(&self, _node: GridlessNode) -> Vec<(GridlessNode, Decimal)> {
        Vec::new()
    }

    fn get_direct_path(&self, waypoints: Vec<GridlessNode>) -> Vec<GridlessNode> {
        waypoints
    }

    fn get_node(
        &self,
        ElevatedPoint { x, y, elevation }: ElevatedPoint,
        _token_shape: &TokenSquareShape,
    ) -> GridlessNode {
        GridlessNode {
            i: y.floor() as i32,
            j: x.floor() as i32,
            k: ((elevation / self.distance) * (self.size as f64) + crate::EPSILON).floor() as i32,
        }
    }

    fn get_node_center_point(&self, GridlessNode { i, j, k }: GridlessNode) -> ElevatedPoint {
        ElevatedPoint { x: j as f64, y: i as f64, elevation: ((k as f64) / (self.size as f64)) * self.distance }
    }

    fn get_node_top_left_point(&self, GridlessNode { i, j, k }: GridlessNode) -> ElevatedPoint {
        ElevatedPoint { x: j as f64, y: i as f64, elevation: ((k as f64) / (self.size as f64)) * self.distance }
    }

    fn get_occupied_grid_space_offsets(
        &self,
        _offset: GridOffset3D,
        _token_shape: &TokenSquareShape,
    ) -> Vec<GridOffset3D> {
        Vec::new()
    }

    fn get_offset(
        &self,
        ElevatedPoint { x, y, elevation }: ElevatedPoint,
        _token_shape: &TokenSquareShape,
    ) -> GridOffset3D {
        GridOffset3D {
            i: y.floor() as i32,
            j: x.floor() as i32,
            k: ((elevation / self.distance) * (self.size as f64) + crate::EPSILON).floor() as i32,
        }
    }

    fn get_offset_center_point(&self, GridOffset3D { i, j, k }: GridOffset3D) -> ElevatedPoint {
        ElevatedPoint { x: j as f64, y: i as f64, elevation: ((k as f64) / (self.size as f64)) * self.distance }
    }

    fn get_offset_top_left_point(&self, GridOffset3D { i, j, k }: GridOffset3D) -> ElevatedPoint {
        ElevatedPoint { x: j as f64, y: i as f64, elevation: ((k as f64) / (self.size as f64)) * self.distance }
    }

    fn get_token_center_point(
        &self,
        ElevatedPoint { x, y, elevation }: ElevatedPoint,
        TokenSquareShape { offsets: _, points: _, center, anchor: _, width: _, height: _ }: &TokenSquareShape,
    ) -> ElevatedPoint {
        ElevatedPoint { x: x + center.x * (self.size as f64), y: y + center.y * (self.size as f64), elevation }
    }

    fn get_token_shape(&self, width: f64, height: f64, _shape: TokenShapeType) -> TokenSquareShape {
        let width = (width * 2.0).round() / 2.0;
        let height = (height * 2.0).round() / 2.0;

        TokenSquareShape {
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

    fn measure_path(&self, _waypoints: Vec<GridlessNode>) -> Decimal {
        dec!(0)
    }

    fn simplify_path(&self, _path: Vec<GridlessNode>) -> Vec<GridlessNode> {
        Vec::new()
    }

    fn test_adjacency(&self, _a: GridlessNode, _b: GridlessNode) -> bool {
        false
    }

    fn test_diagonal(&self, _a: GridlessNode, _b: GridlessNode) -> bool {
        false
    }
}

impl AStar<GridlessNode, TokenSquareShape> for GridlessGrid {
    fn find_path(
        &self,
        _cancellation_token: &crate::exports::CancellationToken,
        _waypoints: Vec<TokenMovementWaypoint>,
        _token: &TokenDocument,
        _scene_rect: &Rect,
        _fog_manager: Option<&FogManager>,
        _region_manager: &RegionManager,
        _wall_manager: &WallManager,
        _grid_measure_path_result: &GridMeasurePathResult,
    ) -> Vec<TokenMovementWaypoint> {
        Vec::new()
    }
}
