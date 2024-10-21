use crate::{
    exports::{Image, TokenShape, Walls},
    traits::Node,
    types::{Point, Rectangle},
};
use rapier2d::prelude::Polyline;
use wasm_bindgen::JsValue;

pub trait BaseGrid<T: Node> {
    fn get_adjacent_nodes(
        &self,
        node: &T,
        bounds: &Option<Rectangle>,
        walls: &Option<Walls>,
        explored: &Option<Image>,
        token_shape: &TokenShape,
    ) -> Vec<(T, u32)>;
    fn get_center_point(&self, node: &T) -> Point;
    fn get_node(&self, point: Point) -> T;
    fn get_token_shape(&self, token: JsValue) -> TokenShape;
}
