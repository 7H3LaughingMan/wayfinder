use std::{collections::HashMap, ops::RangeInclusive};

use crate::{
    enums::TokenShapeType,
    exports::Walls,
    traits::Node,
    types::{ElevatedPoint, GridOffset2D, GridOffset3D, Rectangle, TokenDocument},
};

pub trait BaseGrid<NodeType: Node, TokenShape> {
    fn convert_node_to_offset(&self, node: NodeType) -> GridOffset3D;
    fn convert_offset_to_node(&self, offset: GridOffset3D) -> NodeType;
    fn get_adjacent_nodes(
        &self,
        node: &NodeType,
        token_shape: &TokenShape,
        elevation_range: &RangeInclusive<i32>,
        bounds: &Rectangle,
        walls: &Walls,
        explored: &Option<HashMap<GridOffset2D, bool>>,
    ) -> Vec<(NodeType, u32)>;
    fn get_node(&self, point: ElevatedPoint, token_shape: &TokenShape) -> NodeType;
    fn get_node_center_point(&self, node: &NodeType) -> ElevatedPoint;
    fn get_node_top_left_point(&self, node: &NodeType) -> ElevatedPoint;
    fn get_occupied_grid_space_offsets(&self, offset: GridOffset3D, token_shape: &TokenShape) -> Vec<GridOffset3D>;
    fn get_offset(&self, point: ElevatedPoint, token_shape: &TokenShape) -> GridOffset3D;
    fn get_offset_center_point(&self, offset: GridOffset3D) -> ElevatedPoint;
    fn get_offset_top_left_point(&self, offset: GridOffset3D) -> ElevatedPoint;
    fn get_token_center_point(&self, point: ElevatedPoint, token_shape: &TokenShape) -> ElevatedPoint;
    fn get_token_shape(&self, width: f64, height: f64, shape: TokenShapeType) -> TokenShape;
}
