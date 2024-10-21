use crate::{
    exports::{Image, TokenShape, Walls},
    types::{Point, Rectangle},
};

pub trait AStar {
    fn find_path(
        &self,
        path: Vec<Point>,
        goal: Point,
        bounds: &Option<Rectangle>,
        walls: &Option<Walls>,
        explored: &Option<Image>,
        token_shape: &Option<TokenShape>,
    ) -> Vec<Point>;
}
