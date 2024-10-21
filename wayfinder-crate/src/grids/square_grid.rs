use std::collections::HashMap;

use crate::{
    exports::{image, Image, TokenShape, Walls},
    log,
    nodes::SquareNode,
    traits::{AStar, BaseGrid, JsDeserialize, JsHelper, Node},
    types::{Point, Rectangle},
};
use js_sys::Math::log;
use pathfinding::prelude::{build_path, dijkstra_all};
use rapier2d::prelude::Polyline;
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub struct SquareGrid {
    pub size: i32,
    start: Option<SquareNode>,
    map: HashMap<SquareNode, (SquareNode, u32)>,
}

impl SquareGrid {
    pub fn new(size: i32) -> Self {
        Self { size, start: None, map: HashMap::new() }
    }
}

impl BaseGrid<SquareNode> for SquareGrid {
    fn get_adjacent_nodes(
        &self,
        node: &SquareNode,
        bounds: &Option<Rectangle>,
        walls: &Option<Walls>,
        explored: &Option<Image>,
        shape: &TokenShape,
    ) -> Vec<(SquareNode, u32)> {
        node.get_neighbors()
            .into_iter()
            .filter(|(neighbor, _cost)| {
                if let Some(bounds) = bounds {
                    bounds.contains(self.get_center_point(neighbor) + shape.offset)
                } else {
                    true
                }
            })
            .filter(|(neighbor, _cost)| {
                if let Some(explored) = explored {
                    explored.check_pixel(self.get_center_point(neighbor) + shape.offset)
                } else {
                    true
                }
            })
            .filter(|(neighbor, _cost)| {
                if let Some(walls) = walls {
                    !walls.check_collision(
                        self.get_center_point(node) + shape.offset,
                        self.get_center_point(neighbor) + shape.offset,
                        &shape.polyline,
                    )
                } else {
                    true
                }
            })
            .collect()
    }

    fn get_center_point(&self, SquareNode { i, j, d: _ }: &SquareNode) -> Point {
        let half_size = (self.size as f32) / 2.0;
        Point { x: ((j * self.size) as f32) + half_size, y: ((i * self.size) as f32) + half_size }
    }

    fn get_node(&self, point: Point) -> SquareNode {
        let size = self.size as f32;
        SquareNode { i: (point.y / size).floor() as i32, j: (point.x / size).floor() as i32, d: false }
    }

    fn get_token_shape(&self, token: JsValue) -> TokenShape {
        let token_width: f32;
        let token_height: f32;

        if token.is_object() {
            token_width = f32::max(JsDeserialize::from_value(token.get("document").get("width")), 1.0);
            token_height = f32::max(JsDeserialize::from_value(token.get("document").get("height")), 1.0);
        } else {
            token_width = 1.0;
            token_height = 1.0;
        }

        let width = token_width * (self.size as f32);
        let height = token_height * (self.size as f32);

        let offset = if token_width % 2.0 != 1.0 {
            Point { x: (self.size as f32) / 2.0, y: (self.size as f32) / 2.0 }
        } else {
            Point { x: 0.0, y: 0.0 }
        };

        TokenShape::new(
            Point { x: 0.0, y: 0.0 },
            offset,
            vec![
                Point { x: -(width / 2.0), y: -(height / 2.0) },
                Point { x: (width / 2.0), y: -(height / 2.0) },
                Point { x: (width / 2.0), y: (height / 2.0) },
                Point { x: -(width / 2.0), y: (height / 2.0) },
            ],
        )
    }
}

impl AStar for SquareGrid {
    fn find_path(
        &self,
        path: Vec<Point>,
        goal: Point,
        bounds: &Option<Rectangle>,
        walls: &Option<Walls>,
        explored: &Option<Image>,
        token_shape: &Option<TokenShape>,
    ) -> Vec<Point> {
        let shape = if let Some(token_shape) = token_shape {
            token_shape.shrink(Point { x: self.size as f32, y: self.size as f32 } * Point { x: 0.45, y: 0.45 })
        } else {
            self.get_token_shape(JsValue::NULL)
                .shrink(Point { x: self.size as f32, y: self.size as f32 } * Point { x: 0.45, y: 0.45 })
        };

        let mut path: Vec<SquareNode> = path.into_iter().map(|point| self.get_node(point - shape.offset)).collect();

        if path.is_empty() {
            return Vec::new();
        }

        for idx in 1..path.len() {
            let (left, right) = path.split_at_mut(idx);
            right[0].from(&left[idx - 1]);
        }

        let start_node = *path.last().unwrap();
        let end_node = self.get_node(goal - shape.offset);

        if start_node.at_node(&end_node) {
            return Vec::new();
        }

        let result = pathfinding::prelude::astar(
            &start_node,
            |node| self.get_adjacent_nodes(node, bounds, walls, explored, &shape),
            |node| node.get_distance(&end_node),
            |node| node.at_node(&end_node),
        );

        if let Some((nodes, _cost)) = result {
            return nodes.iter().map(|node| self.get_center_point(node) + shape.offset).collect();
        }

        Vec::new()
    }
}
