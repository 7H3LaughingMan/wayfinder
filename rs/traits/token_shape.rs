use crate::types::Point;

pub trait TokenShape {
    fn get_points(&self) -> Vec<Point>;
    fn get_center(&self) -> Point;
    fn get_anchor(&self) -> Point;
    fn get_width(&self) -> f64;
    fn get_height(&self) -> f64;
}
