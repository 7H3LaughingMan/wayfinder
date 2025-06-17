use crate::types::{GridOffset2D, Point};

#[derive(Clone)]
#[derive(Debug)]
pub struct TokenHexagonalShapeData {
    pub even_offsets: Vec<GridOffset2D>,
    pub odd_offsets: Vec<GridOffset2D>,
    pub points: Vec<Point>,
    pub center: Point,
    pub anchor: Point,
    pub width: f64,
    pub height: f64,
}

#[derive(Clone)]
#[derive(Debug)]
pub struct TokenSquareShapeData {
    pub offsets: Vec<GridOffset2D>,
    pub points: Vec<Point>,
    pub center: Point,
    pub anchor: Point,
    pub width: f64,
    pub height: f64,
}
