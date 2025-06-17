use wasm_bindgen::JsValue;

use crate::traits::{JsDeserialize, JsSerialize};

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub enum TokenShapeType {
    Ellipse1 = 0,
    Ellipse2 = 1,
    Trapezoid1 = 2,
    Trapezoid2 = 3,
    Rectangle1 = 4,
    Rectangle2 = 5,
}

impl JsDeserialize for TokenShapeType {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        let value = i32::from_js(data);

        match value {
            0 => TokenShapeType::Ellipse1,
            1 => TokenShapeType::Ellipse2,
            2 => TokenShapeType::Trapezoid1,
            3 => TokenShapeType::Trapezoid2,
            4 => TokenShapeType::Rectangle1,
            5 => TokenShapeType::Rectangle2,
            _ => panic!("Unknown Token Shape - {value}"),
        }
    }
}

impl JsSerialize for TokenShapeType {
    fn to_value(value: Self) -> wasm_bindgen::JsValue {
        JsValue::from_f64(match value {
            TokenShapeType::Ellipse1 => 0.0,
            TokenShapeType::Ellipse2 => 1.0,
            TokenShapeType::Trapezoid1 => 2.0,
            TokenShapeType::Trapezoid2 => 3.0,
            TokenShapeType::Rectangle1 => 4.0,
            TokenShapeType::Rectangle2 => 5.0,
        })
    }
}
