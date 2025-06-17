use crate::traits::{JsDeserialize, JsHelper, JsSerialize};

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Hash)]
#[derive(PartialEq, Eq)]
pub struct HexagonalGridCube2D {
    pub q: i32,
    pub r: i32,
    pub s: i32,
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Hash)]
#[derive(PartialEq, Eq)]
pub struct HexagonalGridCube3D {
    pub q: i32,
    pub r: i32,
    pub s: i32,
    pub k: i32,
}

impl From<HexagonalGridCube3D> for HexagonalGridCube2D {
    fn from(HexagonalGridCube3D { q, r, s, k: _ }: HexagonalGridCube3D) -> Self {
        HexagonalGridCube2D { q, r, s }
    }
}

impl From<HexagonalGridCube2D> for HexagonalGridCube3D {
    fn from(HexagonalGridCube2D { q, r, s }: HexagonalGridCube2D) -> Self {
        HexagonalGridCube3D { q, r, s, k: 0 }
    }
}

impl JsDeserialize for HexagonalGridCube2D {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        HexagonalGridCube2D { q: data.get_value("q"), r: data.get_value("r"), s: data.get_value("s") }
    }
}

impl JsDeserialize for HexagonalGridCube3D {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        HexagonalGridCube3D {
            q: data.get_value("q"),
            r: data.get_value("r"),
            s: data.get_value("s"),
            k: data.get_value("k"),
        }
    }
}

impl JsSerialize for HexagonalGridCube2D {
    fn to_value(value: Self) -> wasm_bindgen::JsValue {
        let object = js_sys::Object::new();

        object.set("q", JsSerialize::to_value(value.q));
        object.set("r", JsSerialize::to_value(value.r));
        object.set("s", JsSerialize::to_value(value.s));

        return object.into();
    }
}

impl JsSerialize for HexagonalGridCube3D {
    fn to_value(value: Self) -> wasm_bindgen::JsValue {
        let object = js_sys::Object::new();

        object.set("q", JsSerialize::to_value(value.q));
        object.set("r", JsSerialize::to_value(value.r));
        object.set("s", JsSerialize::to_value(value.s));
        object.set("k", JsSerialize::to_value(value.k));

        return object.into();
    }
}
