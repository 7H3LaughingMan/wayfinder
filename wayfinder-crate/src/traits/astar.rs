use crate::{
    exports::{Image, Walls},
    types::{Point, Rectangle},
};

pub trait AStar {
    fn find_path(
        &self,
        path: Vec<Point>,
        goal: Point,
        offset: Point,
        bounds: &Rectangle,
        walls: &Walls,
        explored: &Option<Image>,
    ) -> Vec<Point>;
}
