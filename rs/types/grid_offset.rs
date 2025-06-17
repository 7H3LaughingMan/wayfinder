use crate::traits::{JsDeserialize, JsHelper, JsSerialize};
use std::cmp::Ordering;

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Hash)]
#[derive(PartialEq, Eq)]
pub struct GridOffset2D {
    pub i: i32,
    pub j: i32,
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Hash)]
#[derive(PartialEq, Eq)]
pub struct GridOffset3D {
    pub i: i32,
    pub j: i32,
    pub k: i32,
}

impl From<GridOffset3D> for GridOffset2D {
    fn from(GridOffset3D { i, j, k: _ }: GridOffset3D) -> Self {
        GridOffset2D { i, j }
    }
}

impl From<GridOffset2D> for GridOffset3D {
    fn from(GridOffset2D { i, j }: GridOffset2D) -> Self {
        GridOffset3D { i, j, k: 0 }
    }
}

impl JsDeserialize for GridOffset2D {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        GridOffset2D { i: data.get_value("i"), j: data.get_value("j") }
    }
}

impl JsDeserialize for GridOffset3D {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        GridOffset3D { i: data.get_value("i"), j: data.get_value("j"), k: data.get_value("k") }
    }
}

impl JsSerialize for GridOffset2D {
    fn to_value(value: Self) -> wasm_bindgen::JsValue {
        let object = js_sys::Object::new();

        object.set("i", JsSerialize::to_value(value.i));
        object.set("j", JsSerialize::to_value(value.j));

        return object.into();
    }
}

impl JsSerialize for GridOffset3D {
    fn to_value(value: Self) -> wasm_bindgen::JsValue {
        let object = js_sys::Object::new();

        object.set("i", JsSerialize::to_value(value.i));
        object.set("j", JsSerialize::to_value(value.j));
        object.set("k", JsSerialize::to_value(value.k));

        return object.into();
    }
}

impl Ord for GridOffset2D {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.j.cmp(&other.j) {
            Ordering::Equal => {}
            ord => return ord,
        }
        self.i.cmp(&other.i)
    }
}

impl Ord for GridOffset3D {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.j.cmp(&other.j) {
            Ordering::Equal => {}
            ord => return ord,
        }
        match self.i.cmp(&other.i) {
            Ordering::Equal => {}
            ord => return ord,
        }
        self.k.cmp(&other.k)
    }
}

impl PartialOrd for GridOffset2D {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for GridOffset3D {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
