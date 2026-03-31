use itertools::Itertools;
use js_sys::{Array, Number, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = Object,
        js_name = Polygon,
        js_namespace = PIXI,
        typescript_type = "PIXI.Polygon"
    )]
    pub type JsPolygon;

    #[wasm_bindgen(constructor, variadic, js_class = Polygon, js_namespace = PIXI)]
    pub fn new(points: Array<Number>) -> JsPolygon;

    #[wasm_bindgen(method, getter = closeStroke)]
    pub fn close_stroke(this: &JsPolygon) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn points(this: &JsPolygon) -> Array<Number>;

    #[wasm_bindgen(method)]
    pub fn clone(this: &JsPolygon) -> JsPolygon;

    #[wasm_bindgen(method)]
    pub fn contains(this: &JsPolygon, x: f64, y: f64) -> bool;
}

impl From<crate::types::shapes::Polygon> for JsPolygon {
    fn from(value: crate::types::shapes::Polygon) -> Self {
        JsPolygon::new(value.points.iter().map(|n| Number::from(*n)).collect::<Array>().unchecked_into())
    }
}

impl From<geo::Polygon> for JsPolygon {
    fn from(value: geo::Polygon) -> Self {
        JsPolygon::new(
            value
                .exterior()
                .coords()
                .dropping_back(1)
                .flat_map(|geo::Coord { x, y }| [Number::from(*x), Number::from(*y)])
                .collect::<Array>()
                .unchecked_into(),
        )
    }
}

impl From<JsPolygon> for geo::Polygon {
    fn from(value: JsPolygon) -> Self {
        geo::Polygon::new(value.points().iter().map(|n| n.value_of()).tuples::<(f64, f64)>().collect(), Vec::new())
    }
}
