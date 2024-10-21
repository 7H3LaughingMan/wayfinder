use crate::{
    enums::Grid,
    exports::{Image, TokenShape, Walls},
    traits::{AStar, JsDeserialize, JsSerialize},
    types::{Point, Rectangle},
};
use pathfinding::grid;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "SquareGrid | HexagonalGrid | GridlessGrid")]
    pub type JsGrid;

    #[wasm_bindgen(typescript_type = "Point")]
    pub type JsPoint;

    #[wasm_bindgen(typescript_type = "Rectangle | undefined")]
    pub type JsRectangle;

    #[wasm_bindgen(typescript_type = "Token")]
    pub type JsToken;

    #[wasm_bindgen(typescript_type = "WallDocument")]
    pub type JsWallDocument;
}

#[wasm_bindgen]
pub struct Wayfinder {
    bounds: Option<Rectangle>,
    explored: Option<Image>,
    grid: Option<Grid>,
    token_shape: Option<TokenShape>,
    walls: Option<Walls>,
}

#[wasm_bindgen]
impl Wayfinder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Wayfinder {
        Wayfinder { bounds: None, explored: None, grid: None, token_shape: None, walls: None }
    }

    #[wasm_bindgen(js_name = addBounds)]
    pub fn add_bounds(&mut self, bounds: JsRectangle) {
        self.bounds = Some(JsDeserialize::from_value(bounds.into()));
    }

    #[wasm_bindgen(js_name = addExplored)]
    pub fn add_explored(&mut self, pixels: Vec<u8>, bounds: JsRectangle, scaled_bounds: JsRectangle) {
        self.explored = Some(Image::new(
            bytemuck::allocation::cast_vec(pixels),
            JsDeserialize::from_value(bounds.into()),
            JsDeserialize::from_value(scaled_bounds.into()),
        ));
    }

    #[wasm_bindgen(js_name = addGrid)]
    pub fn add_grid(&mut self, grid: JsGrid) {
        self.grid = Some(JsDeserialize::from_value(grid.into()));
    }

    #[wasm_bindgen(js_name = addToken)]
    pub fn add_token(&mut self, token: JsToken) {
        if let Some(grid) = &self.grid {
            self.token_shape = Some(grid.get_token_shape(token.into()));
        }
    }

    #[wasm_bindgen(js_name = addWalls)]
    pub fn add_walls(&mut self, wall_documents: Vec<JsWallDocument>) {
        self.walls = Some(Walls::new(wall_documents));
    }

    #[wasm_bindgen(js_name = findPath)]
    pub fn find_path(&mut self, path: Vec<JsPoint>, goal: JsPoint) -> Vec<JsPoint> {
        let path = JsDeserialize::from_value(path.into());
        let goal = JsDeserialize::from_value(goal.into());

        let path = match &self.grid {
            Some(grid) => match grid {
                Grid::Gridless(gridless_grid) => {
                    gridless_grid.find_path(path, goal, &self.bounds, &self.walls, &self.explored, &self.token_shape)
                }
                Grid::Square(square_grid) => {
                    square_grid.find_path(path, goal, &self.bounds, &self.walls, &self.explored, &self.token_shape)
                }
                Grid::Hexagonal(hexagonal_grid) => {
                    hexagonal_grid.find_path(path, goal, &self.bounds, &self.walls, &self.explored, &self.token_shape)
                }
            },
            None => Vec::new(),
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
