use crate::{
    exports::{Image, Walls},
    traits::Node,
    types::{Point, Rectangle},
};
use wasm_bindgen::JsValue;

pub trait BaseGrid<T: Node> {
    fn get_adjacent_nodes(
        &self,
        node: &T,
        offset: &Point,
        bounds: &Rectangle,
        walls: &Walls,
        explored: &Option<Image>,
    ) -> Vec<(T, u32)>;
    fn get_center_point(&self, node: &T) -> Point;
    fn get_node(&self, point: Point) -> T;
}
