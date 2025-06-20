use crate::{
    traits::TokenShape,
    types::{GridOffset2D, Point},
};

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

impl TokenShape for TokenHexagonalShapeData {
    fn get_points(&self) -> Vec<Point> {
        self.points.clone()
    }

    fn get_center(&self) -> Point {
        self.center
    }

    fn get_anchor(&self) -> Point {
        self.anchor
    }

    fn get_width(&self) -> f64 {
        self.width
    }

    fn get_height(&self) -> f64 {
        self.height
    }
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

impl TokenShape for TokenSquareShapeData {
    fn get_points(&self) -> Vec<Point> {
        self.points.clone()
    }

    fn get_center(&self) -> Point {
        self.center
    }

    fn get_anchor(&self) -> Point {
        self.anchor
    }

    fn get_width(&self) -> f64 {
        self.width
    }

    fn get_height(&self) -> f64 {
        self.height
    }
}
