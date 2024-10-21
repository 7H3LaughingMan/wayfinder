use crate::traits::{JsDeserialize, JsHelper};
use crate::{
    exports::TokenShape,
    grids::{GridlessGrid, HexagonalGrid, SquareGrid},
    traits::BaseGrid,
    types::Point,
};
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub enum Grid {
    Gridless(GridlessGrid),
    Square(SquareGrid),
    Hexagonal(HexagonalGrid),
}

impl Grid {
    pub fn get_token_shape(&self, token: JsValue) -> TokenShape {
        match self {
            Grid::Gridless(gridless_grid) => gridless_grid.get_token_shape(token),
            Grid::Square(square_grid) => square_grid.get_token_shape(token),
            Grid::Hexagonal(hexagonal_grid) => hexagonal_grid.get_token_shape(token),
        }
    }

    pub fn get_size(&self) -> Point {
        match self {
            Grid::Gridless(gridless_grid) => Point { x: gridless_grid.size as f32, y: gridless_grid.size as f32 },
            Grid::Square(square_grid) => Point { x: square_grid.size as f32, y: square_grid.size as f32 },
            Grid::Hexagonal(hexagonal_grid) => Point { x: hexagonal_grid.size_x, y: hexagonal_grid.size_y },
        }
    }
}

impl JsDeserialize for Grid {
    fn from_value(value: JsValue) -> Self {
        let r#type = JsDeserialize::from_value(value.get("type"));

        match r#type {
            0 => Grid::Gridless(GridlessGrid { size: JsDeserialize::from_value(value.get("size")) }),
            1 => Grid::Square(SquareGrid::new(JsDeserialize::from_value(value.get("size")))),
            2..=5 => Grid::Hexagonal(HexagonalGrid {
                size: JsDeserialize::from_value(value.get("size")),
                size_x: JsDeserialize::from_value(value.get("sizeX")),
                size_y: JsDeserialize::from_value(value.get("sizeY")),
                columns: JsDeserialize::from_value(value.get("columns")),
                even: JsDeserialize::from_value(value.get("even")),
            }),
            type_ => panic!("Unknown Grid Type - {type_}"),
        }
    }
}
