use crate::{
    enums::Grid,
    exports::{Image, Walls},
    traits::{AStar, JsDeserialize, JsDeserializeVector, JsSerialize},
    types::{GLTexture, Point, Rectangle},
};
use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(
        typescript_type = "foundry.grid.SquareGrid | foundry.grid.HexagonalGrid | foundry.grid.GridlessGrid"
    )]
    pub type JsGrid;

    #[derive(Debug)]
    #[wasm_bindgen(typescript_type = "Point")]
    pub type JsPoint;

    #[derive(Debug)]
    #[wasm_bindgen(typescript_type = "Rectangle")]
    pub type JsRectangle;

    #[derive(Debug)]
    #[wasm_bindgen(typescript_type = "Token")]
    pub type JsToken;

    #[derive(Debug)]
    #[wasm_bindgen(typescript_type = "WallDocument")]
    pub type JsWallDocument;

    #[derive(Debug)]
    #[wasm_bindgen(typescript_type = "GLTexture")]
    pub type JsGLTexture;
}

#[wasm_bindgen]
pub struct Wayfinder {
    bounds: Rectangle,
    explored: Option<Image>,
    grid: Grid,
    offset: Option<Point>,
    walls: Walls,
}

#[wasm_bindgen]
impl Wayfinder {
    #[wasm_bindgen(constructor)]
    pub fn new(bounds: JsRectangle, grid: JsGrid, wall_documents: Vec<JsWallDocument>) -> Wayfinder {
        let bounds = Rectangle::from_js(bounds);
        let grid = Grid::from_js(grid);
        let walls = Walls::new(bounds, wall_documents);

        Wayfinder { bounds, explored: None, grid, offset: None, walls }
    }

    #[wasm_bindgen(js_name = addWall)]
    pub fn add_wall(&mut self, wall_document: JsWallDocument) {
        self.walls.add_wall(wall_document);
    }

    #[wasm_bindgen(js_name = deleteWall)]
    pub fn delete_wall(&mut self, wall_document: JsWallDocument) {
        self.walls.delete_wall(wall_document);
    }

    #[wasm_bindgen(js_name = updateWall)]
    pub fn update_wall(&mut self, wall_document: JsWallDocument) {
        self.walls.update_wall(wall_document);
    }

    #[wasm_bindgen(js_name = addExplored)]
    pub fn add_explored(&mut self, gl: WebGl2RenderingContext, texture: JsGLTexture, bounds: JsRectangle) {
        self.explored = Some(Image::new(gl, GLTexture::from_js(texture), Rectangle::from_js(bounds)));
    }

    #[wasm_bindgen(js_name = addOffset)]
    pub fn add_offset(&mut self, offset: Option<JsPoint>) {
        match offset {
            Some(point) => self.offset = Some(Point::from_js(point)),
            None => self.offset = None,
        }
    }

    #[wasm_bindgen(js_name = findPath)]
    pub fn find_path(&mut self, path: Vec<JsPoint>, goal: JsPoint) -> Vec<JsPoint> {
        let path = Point::from_js_vector(path);
        let goal = Point::from_js(goal);
        let offset = match self.offset {
            Some(point) => point,
            None => Point::new(0.0, 0.0),
        };

        let path = match &self.grid {
            Grid::Gridless(gridless_grid) => {
                gridless_grid.find_path(path, goal, offset, &self.bounds, &self.walls, &self.explored)
            }
            Grid::Square(square_grid) => {
                square_grid.find_path(path, goal, offset, &self.bounds, &self.walls, &self.explored)
            }
            Grid::Hexagonal(hexagonal_grid) => {
                hexagonal_grid.find_path(path, goal, offset, &self.bounds, &self.walls, &self.explored)
            }
        };

        Wayfinder::simplify_path(path).iter().map(|&p| JsSerialize::to_value(p).into()).collect()
    }

    fn simplify_path(nodes: Vec<Point>) -> Vec<Point> {
        let mut nodes = nodes.clone();
        let mut i = 0;

        while i + 2 < nodes.len() {
            let point_1 = nodes[i];
            let point_2 = nodes[i + 1];
            let point_3 = nodes[i + 2];

            let ray_1 = point_2 - point_1;
            let ray_2 = point_3 - point_2;

            let angle = ray_2.y.atan2(ray_2.x) - ray_1.y.atan2(ray_1.x);

            if -0.08727 < angle && angle < 0.08727 {
                nodes.remove(i + 1);
            } else {
                i += 1;
            }
        }

        nodes
    }
}
