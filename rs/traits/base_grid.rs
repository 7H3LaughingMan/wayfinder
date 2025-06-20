use crate::{
    enums::TokenShapeType,
    exports::{Fog, Walls},
    traits::{Node, TokenShape},
    types::{ElevatedPoint, GridOffset3D, Rectangle},
};
use std::ops::RangeInclusive;

pub trait BaseGrid<N: Node, T: TokenShape> {
    fn convert_node_to_offset(&self, node: N) -> GridOffset3D;
    fn convert_offset_to_node(&self, offset: GridOffset3D) -> N;
    fn get_adjacent_nodes(
        &self,
        node: &N,
        token_shape: &T,
        elevation_range: &RangeInclusive<i32>,
        bounds: &Rectangle,
        walls: &Walls,
        fog: &Option<Fog>,
    ) -> Vec<(N, u32)> {
        node.get_neighbors()
            .into_iter()
            .filter(|(neighbor, _cost)| elevation_range.contains(&neighbor.get_elevation()))
            .filter(|(neighbor, _cost)| bounds.contains_point(self.get_node_center_point(neighbor).into()))
            .filter(|(neighbor, _cost)| {
                if let Some(fog) = fog {
                    fog.is_point_explored(self.get_node_center_point(neighbor).into())
                } else {
                    true
                }
            })
            .filter(|(neighbor, _cost)| {
                if token_shape.get_width() > 1.0 || token_shape.get_height() > 1.0 {
                    if walls.check_point(
                        self.get_token_center_point(self.get_node_top_left_point(neighbor), token_shape).into(),
                    ) {
                        return false;
                    }
                }

                !walls.check_collisions(
                    self.get_occupied_grid_space_offsets(self.convert_node_to_offset(*node), token_shape)
                        .iter()
                        .map(|offset| self.get_offset_center_point(*offset).into())
                        .zip(
                            self.get_occupied_grid_space_offsets(self.convert_node_to_offset(*neighbor), token_shape)
                                .iter()
                                .map(|offset| self.get_offset_center_point(*offset).into()),
                        )
                        .collect(),
                )
            })
            .collect()
    }
    fn get_node(&self, point: ElevatedPoint, token_shape: &T) -> N;
    fn get_node_center_point(&self, node: &N) -> ElevatedPoint;
    fn get_node_top_left_point(&self, node: &N) -> ElevatedPoint;
    fn get_occupied_grid_space_offsets(&self, offset: GridOffset3D, token_shape: &T) -> Vec<GridOffset3D>;
    fn get_offset(&self, point: ElevatedPoint, token_shape: &T) -> GridOffset3D;
    fn get_offset_center_point(&self, offset: GridOffset3D) -> ElevatedPoint;
    fn get_offset_top_left_point(&self, offset: GridOffset3D) -> ElevatedPoint;
    fn get_token_center_point(&self, point: ElevatedPoint, token_shape: &T) -> ElevatedPoint;
    fn get_token_shape(&self, width: f64, height: f64, shape: TokenShapeType) -> T;
    fn simplify_path(&self, path: Vec<N>) -> Vec<N>;
}
