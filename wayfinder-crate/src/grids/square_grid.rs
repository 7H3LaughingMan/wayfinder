use std::collections::HashMap;

use crate::{
    exports::{image, Image, Walls},
    log,
    nodes::SquareNode,
    traits::{AStar, BaseGrid, JsDeserialize, JsHelper, Node},
    types::{Point, Rectangle},
};
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
        offset: &Point,
        bounds: &Rectangle,
        walls: &Walls,
        explored: &Option<Image>,
    ) -> Vec<(SquareNode, u32)> {
        node.get_neighbors()
            .into_iter()
            .filter(|(neighbor, _cost)| bounds.contains_point(self.get_center_point(neighbor) + *offset))
            .filter(|(neighbor, _cost)| {
                if let Some(explored) = explored {
                    explored.check_pixel(self.get_center_point(neighbor) + *offset)
                } else {
                    true
                }
            })
            .filter(|(neighbor, _cost)| {
                !walls.check_collision(self.get_center_point(node) + *offset, self.get_center_point(neighbor) + *offset)
            })
            .collect()
    }

    fn get_center_point(&self, SquareNode { i, j, d: _ }: &SquareNode) -> Point {
        let half_size = (self.size as f64) / 2.0;
        Point { x: ((j * self.size) as f64) + half_size, y: ((i * self.size) as f64) + half_size }
    }

    fn get_node(&self, point: Point) -> SquareNode {
        let size = self.size as f64;
        SquareNode { i: (point.y / size).floor() as i32, j: (point.x / size).floor() as i32, d: false }
    }
}

impl AStar for SquareGrid {
    fn find_path(
        &self,
        path: Vec<Point>,
        goal: Point,
        offset: Point,
        bounds: &Rectangle,
        walls: &Walls,
        explored: &Option<Image>,
    ) -> Vec<Point> {
        let mut path: Vec<SquareNode> = path.into_iter().map(|point| self.get_node(point - offset)).collect();

        if path.is_empty() {
            return Vec::new();
        }

        for idx in 1..path.len() {
            let (left, right) = path.split_at_mut(idx);
            right[0].from(&left[idx - 1]);
        }

        let start_node = *path.last().unwrap();
        let end_node = self.get_node(goal - offset);

        if start_node.at_node(&end_node) {
            return Vec::new();
        }

        let result = pathfinding::prelude::astar(
            &start_node,
            |node| self.get_adjacent_nodes(node, &offset, bounds, walls, explored),
            |node| node.get_distance(&end_node),
            |node| node.at_node(&end_node),
        );

        if let Some((nodes, _cost)) = result {
            return nodes.iter().map(|node| self.get_center_point(node) + offset).collect();
        }

        Vec::new()
    }
}
