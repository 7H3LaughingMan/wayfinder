use crate::{
    enums::TokenShapeType,
    exports::{Fog, Walls},
    nodes::SquareNode,
    traits::{node, AStar, BaseGrid, Node, SkipLast},
    types::{
        ElevatedPoint, GridMeasurePathResult, GridOffset2D, GridOffset3D, Point, Rectangle, TokenDocument,
        TokenMovementWaypoint, TokenSquareShapeData,
    },
};
use std::{collections::HashMap, ops::RangeInclusive, vec};

#[derive(Debug)]
pub struct SquareGrid {
    pub size: i32,
    pub distance: f64,
}

impl BaseGrid<SquareNode, TokenSquareShapeData> for SquareGrid {
    fn convert_node_to_offset(&self, SquareNode { i, j, k, d: _ }: SquareNode) -> GridOffset3D {
        GridOffset3D { i, j, k }
    }

    fn convert_offset_to_node(&self, GridOffset3D { i, j, k }: GridOffset3D) -> SquareNode {
        SquareNode { i, j, k, d: false }
    }

    fn get_node(
        &self,
        ElevatedPoint { mut x, mut y, elevation }: ElevatedPoint,
        TokenSquareShapeData { offsets: _, points: _, center: _, anchor: _, width, height }: &TokenSquareShapeData,
    ) -> SquareNode {
        x += self.size as f64 * if width.fract() == 0.0 { 0.5 } else { 0.25 };
        y += self.size as f64 * if height.fract() == 0.0 { 0.5 } else { 0.25 };

        SquareNode {
            i: (y / self.size as f64).floor() as i32,
            j: (x / self.size as f64).floor() as i32,
            k: ((elevation / self.distance) + 1e-8).floor() as i32,
            d: false,
        }
    }

    fn get_node_center_point(&self, SquareNode { i, j, k, d: _ }: &SquareNode) -> ElevatedPoint {
        ElevatedPoint {
            x: (*j as f64 + 0.5) * self.size as f64,
            y: (*i as f64 + 0.5) * self.size as f64,
            elevation: (*k as f64 + 0.5) * self.distance,
        }
    }

    fn get_node_top_left_point(&self, SquareNode { i, j, k, d: _ }: &SquareNode) -> ElevatedPoint {
        ElevatedPoint {
            x: *j as f64 * self.size as f64,
            y: *i as f64 * self.size as f64,
            elevation: *k as f64 * self.distance,
        }
    }

    fn get_occupied_grid_space_offsets(
        &self,
        GridOffset3D { i, j, k }: GridOffset3D,
        token_shape: &TokenSquareShapeData,
    ) -> Vec<GridOffset3D> {
        token_shape.offsets.iter().map(|offset| GridOffset3D { i: i + offset.i, j: j + offset.j, k }).collect()
    }

    fn get_offset(
        &self,
        ElevatedPoint { mut x, mut y, elevation }: ElevatedPoint,
        TokenSquareShapeData { offsets: _, points: _, center: _, anchor: _, width, height }: &TokenSquareShapeData,
    ) -> GridOffset3D {
        x += self.size as f64 * if width.fract() == 0.0 { 0.5 } else { 0.25 };
        y += self.size as f64 * if height.fract() == 0.0 { 0.5 } else { 0.25 };

        GridOffset3D {
            i: (y / self.size as f64).floor() as i32,
            j: (x / self.size as f64).floor() as i32,
            k: ((elevation / self.distance) + 1e-8).floor() as i32,
        }
    }

    fn get_offset_center_point(&self, GridOffset3D { i, j, k }: GridOffset3D) -> ElevatedPoint {
        ElevatedPoint {
            x: (j as f64 + 0.5) * self.size as f64,
            y: (i as f64 + 0.5) * self.size as f64,
            elevation: (k as f64 + 0.5) * self.distance,
        }
    }

    fn get_offset_top_left_point(&self, GridOffset3D { i, j, k }: GridOffset3D) -> ElevatedPoint {
        ElevatedPoint {
            x: j as f64 * self.size as f64,
            y: i as f64 * self.size as f64,
            elevation: k as f64 * self.distance,
        }
    }

    fn get_token_shape(&self, width: f64, height: f64, _shape: TokenShapeType) -> TokenSquareShapeData {
        let mut offsets = Vec::<GridOffset2D>::new();
        let width = (width * 2.0).round() / 2.0;
        let height = (height * 2.0).round() / 2.0;

        for i in 0..(height.ceil() as i32) {
            for j in 0..(width.ceil() as i32) {
                offsets.push(GridOffset2D { i, j });
            }
        }

        TokenSquareShapeData {
            offsets,
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

    fn get_token_center_point(
        &self,
        ElevatedPoint { x, y, elevation }: ElevatedPoint,
        TokenSquareShapeData { offsets: _, points: _, center, anchor: _, width: _, height: _ }: &TokenSquareShapeData,
    ) -> ElevatedPoint {
        ElevatedPoint { x: x + (center.x * self.size as f64), y: y + (center.y * self.size as f64), elevation }
    }

    fn simplify_path(&self, path: Vec<SquareNode>) -> Vec<SquareNode> {
        let mut path: Vec<SquareNode> = path.clone();
        let mut i = 0;

        while i + 2 < path.len() {
            let node_1 = path[i];
            let node_2 = path[i + 1];
            let node_3 = path[i + 2];

            let vector_a = GridOffset3D {
                i: (node_2.i - node_1.i).clamp(-1, 1),
                j: (node_2.j - node_1.j).clamp(-1, 1),
                k: (node_2.k - node_1.k).clamp(-1, 1),
            };

            let vector_b = GridOffset3D {
                i: (node_3.i - node_2.i).clamp(-1, 1),
                j: (node_3.j - node_2.j).clamp(-1, 1),
                k: (node_3.k - node_2.k).clamp(-1, 1),
            };

            if vector_a == vector_b {
                path.remove(i + 1);
            } else {
                i += 1;
            }
        }

        path
    }
}

impl AStar<SquareNode, TokenSquareShapeData> for SquareGrid {}
