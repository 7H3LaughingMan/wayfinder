use crate::{
    exports::{Image, Walls},
    nodes::GridlessNode,
    traits::{AStar, BaseGrid, JsDeserialize, JsHelper},
    types::{Point, Rectangle},
};
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub struct GridlessGrid {
    pub size: i32,
}

impl BaseGrid<GridlessNode> for GridlessGrid {
    fn get_adjacent_nodes(
        &self,
        node: &GridlessNode,
        offset: &Point,
        bounds: &Rectangle,
        walls: &Walls,
        explored: &Option<Image>,
    ) -> Vec<(GridlessNode, u32)> {
        let _ = node;
        let _ = offset;
        let _ = bounds;
        let _ = walls;
        let _ = explored;

        Vec::new()
    }

    fn get_center_point(&self, GridlessNode { i, j }: &GridlessNode) -> Point {
        let half_size = (self.size as f64) / 2.0;
        Point { x: ((j * self.size) as f64) + half_size, y: ((i * self.size) as f64) + half_size }
    }

    fn get_node(&self, point: Point) -> GridlessNode {
        let size = self.size as f64;
        GridlessNode { i: (point.y / size).floor() as i32, j: (point.x / size).floor() as i32 }
    }
}

impl AStar for GridlessGrid {
    fn find_path(
        &self,
        path: Vec<Point>,
        goal: Point,
        offset: Point,
        bounds: &Rectangle,
        walls: &Walls,
        explored: &Option<Image>,
    ) -> Vec<Point> {
        let _ = path;
        let _ = goal;
        let _ = offset;
        let _ = bounds;
        let _ = walls;
        let _ = explored;

        Vec::new()
    }
}
