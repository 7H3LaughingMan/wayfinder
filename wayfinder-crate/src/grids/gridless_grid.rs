use crate::{
    exports::{token_shape, Image, TokenShape, Walls},
    nodes::GridlessNode,
    traits::{AStar, BaseGrid, JsDeserialize, JsHelper},
    types::{Point, Rectangle},
};
use rapier2d::prelude::Polyline;
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub struct GridlessGrid {
    pub size: i32,
}

impl BaseGrid<GridlessNode> for GridlessGrid {
    fn get_adjacent_nodes(
        &self,
        node: &GridlessNode,
        bounds: &Option<Rectangle>,
        walls: &Option<Walls>,
        explored: &Option<Image>,
        shape: &TokenShape,
    ) -> Vec<(GridlessNode, u32)> {
        let _ = node;
        let _ = bounds;
        let _ = walls;
        let _ = explored;
        let _ = shape;

        Vec::new()
    }

    fn get_center_point(&self, GridlessNode { i, j }: &GridlessNode) -> Point {
        let half_size = (self.size as f32) / 2.0;
        Point { x: ((j * self.size) as f32) + half_size, y: ((i * self.size) as f32) + half_size }
    }

    fn get_node(&self, point: Point) -> GridlessNode {
        let size = self.size as f32;
        GridlessNode { i: (point.y / size).floor() as i32, j: (point.x / size).floor() as i32 }
    }

    fn get_token_shape(&self, token: JsValue) -> TokenShape {
        let token_width: f32;
        let token_height: f32;

        if token.is_object() {
            token_width = JsDeserialize::from_value(token.get("document").get("width"));
            token_height = JsDeserialize::from_value(token.get("document").get("height"));
        } else {
            token_width = 1.0;
            token_height = 1.0;
        }

        let width = token_width * (self.size as f32);
        let height = token_height * (self.size as f32);

        TokenShape::new(
            Point { x: 0.0, y: 0.0 },
            Point { x: 0.0, y: 0.0 },
            vec![
                Point { x: -(width / 2.0), y: -(height / 2.0) },
                Point { x: (width / 2.0), y: -(height / 2.0) },
                Point { x: (width / 2.0), y: (height / 2.0) },
                Point { x: -(width / 2.0), y: (height / 2.0) },
            ],
        )
    }
}

impl AStar for GridlessGrid {
    fn find_path(
        &self,
        path: Vec<Point>,
        goal: Point,
        bounds: &Option<Rectangle>,
        walls: &Option<Walls>,
        explored: &Option<Image>,
        token_shape: &Option<TokenShape>,
    ) -> Vec<Point> {
        let _ = path;
        let _ = goal;
        let _ = bounds;
        let _ = walls;
        let _ = explored;
        let _ = token_shape;

        Vec::new()
    }
}
