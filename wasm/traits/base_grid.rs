use crate::{
    traits::{Node, TokenShape},
    types::{
        foundry::TokenShapeType,
        wayfinder::{ElevatedPoint, FogManager, GridOffset3D, RegionManager, WallManager},
    },
};
use rust_decimal::Decimal;

pub trait BaseGrid<N: Node, T: TokenShape> {
    fn calculate_cost(
        &self,
        from: N,
        to: N,
        token_shape: &T,
        level: &str,
        fog_manager: Option<&FogManager>,
        region_manager: &RegionManager,
        wall_manager: &WallManager,
    ) -> Option<(N, Decimal)>;
    fn convert_node_to_offset(&self, node: N) -> GridOffset3D;
    fn convert_offset_to_node(&self, offset: GridOffset3D) -> N;
    fn get_adjacent_nodes(&self, node: N) -> Vec<(N, Decimal)>;
    fn get_direct_path(&self, waypoints: Vec<N>) -> Vec<N>;
    fn get_node(&self, point: ElevatedPoint, token_shape: &T) -> N;
    fn get_node_center_point(&self, node: N) -> ElevatedPoint;
    fn get_node_top_left_point(&self, node: N) -> ElevatedPoint;
    fn get_occupied_grid_space_offsets(&self, offset: GridOffset3D, token_shape: &T) -> Vec<GridOffset3D>;
    fn get_offset(&self, point: ElevatedPoint, token_shape: &T) -> GridOffset3D;
    fn get_offset_center_point(&self, offset: GridOffset3D) -> ElevatedPoint;
    fn get_offset_top_left_point(&self, offset: GridOffset3D) -> ElevatedPoint;
    fn get_token_center_point(&self, point: ElevatedPoint, token_shape: &T) -> ElevatedPoint;
    fn get_token_shape(&self, width: f64, height: f64, shape: TokenShapeType) -> T;
    fn measure_path(&self, waypoints: Vec<N>) -> Decimal;
    fn simplify_path(&self, path: Vec<N>) -> Vec<N>;
    fn test_adjacency(&self, a: N, b: N) -> bool;
    fn test_diagonal(&self, a: N, b: N) -> bool;
}
