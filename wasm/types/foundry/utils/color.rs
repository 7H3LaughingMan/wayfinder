use crate::types::wayfinder::Color;
use js_sys::{ArrayTuple, Number};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(
        extends = Number,
        js_name = Color,
        typescript_type = "Color"
    )]
    pub type JsColor;

    #[wasm_bindgen(constructor, js_class = Color)]
    pub fn new(value: JsValue) -> JsColor;

    #[wasm_bindgen(method, getter)]
    pub fn valid(this: &JsColor) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn css(this: &JsColor) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn rgb(this: &JsColor) -> ArrayTuple<(Number, Number, Number)>;

    #[wasm_bindgen(method, getter)]
    pub fn r(this: &JsColor) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn g(this: &JsColor) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn b(this: &JsColor) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn maximum(this: &JsColor) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn minimum(this: &JsColor) -> f64;

    #[wasm_bindgen(method, getter = littleEndian)]
    pub fn little_endian(this: &JsColor) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn hsv(this: &JsColor) -> ArrayTuple<(Number, Number, Number)>;

    #[wasm_bindgen(method, getter)]
    pub fn hsl(this: &JsColor) -> ArrayTuple<(Number, Number, Number)>;

    #[wasm_bindgen(method, getter)]
    pub fn linear(this: &JsColor) -> JsColor;

    #[wasm_bindgen(method, js_name = toJSON)]
    pub fn to_json(this: &JsColor) -> String;

    #[wasm_bindgen(method, js_name = toHTML)]
    pub fn to_html(this: &JsColor) -> String;

    #[wasm_bindgen(method)]
    pub fn equals(this: &JsColor, other: JsColor) -> bool;

    #[wasm_bindgen(method, js_name = equals)]
    pub fn equals_number(this: &JsColor, other: f64) -> bool;

    #[wasm_bindgen(method, js_name = toRGBA)]
    pub fn to_rgba(this: &JsColor) -> String;

    #[wasm_bindgen(method)]
    pub fn mix(this: &JsColor, other: JsColor, weight: f64) -> JsColor;

    #[wasm_bindgen(method)]
    pub fn multiply(this: &JsColor, other: JsColor) -> JsColor;

    #[wasm_bindgen(method, js_name = multiply)]
    pub fn multiply_number(this: &JsColor, other: f64) -> JsColor;

    #[wasm_bindgen(method)]
    pub fn add(this: &JsColor, other: JsColor) -> JsColor;

    #[wasm_bindgen(method, js_name = add)]
    pub fn add_number(this: &JsColor, other: f64) -> JsColor;

    #[wasm_bindgen(method)]
    pub fn subtract(this: &JsColor, other: JsColor) -> JsColor;

    #[wasm_bindgen(method, js_name = subtract)]
    pub fn subtract_number(this: &JsColor, other: f64) -> JsColor;

    #[wasm_bindgen(method)]
    pub fn maximize(this: &JsColor, other: JsColor) -> JsColor;

    #[wasm_bindgen(method, js_name = maximize)]
    pub fn maximize_number(this: &JsColor, other: f64) -> JsColor;

    #[wasm_bindgen(method)]
    pub fn minimize(this: &JsColor, other: JsColor) -> JsColor;

    #[wasm_bindgen(method, js_name = minimize)]
    pub fn minimizee_number(this: &JsColor, other: f64) -> JsColor;

    #[wasm_bindgen(method, js_name = applyRGB)]
    pub fn apply_rgb(this: &JsColor, vec3: &ArrayTuple<(Number, Number, Number)>);

    #[wasm_bindgen(static_method_of = JsColor, js_class = Color, js_name = fromString)]
    pub fn from_string(color: String) -> JsColor;

    #[wasm_bindgen(static_method_of = JsColor, js_class = Color, js_name = fromRGB)]
    pub fn from_rgb(rgb: ArrayTuple<(Number, Number, Number)>) -> JsColor;

    #[wasm_bindgen(static_method_of = JsColor, js_class = Color, js_name = fromRGBvalues)]
    pub fn from_rgb_values(r: f64, g: f64, b: f64) -> JsColor;

    #[wasm_bindgen(static_method_of = JsColor, js_class = Color, js_name = fromHSV)]
    pub fn from_hsv(hsv: ArrayTuple<(Number, Number, Number)>) -> JsColor;

    #[wasm_bindgen(static_method_of = JsColor, js_class = Color, js_name = fromHSL)]
    pub fn from_hsl(hsl: ArrayTuple<(Number, Number, Number)>) -> JsColor;

    #[wasm_bindgen(static_method_of = JsColor, js_class = Color, js_name = fromLinearRGB)]
    pub fn from_linear_rgb(linear: ArrayTuple<(Number, Number, Number)>) -> JsColor;
}

impl js_sys::Iterable for JsColor {
    type Item = Number;
}

impl From<Color> for JsColor {
    fn from(value: Color) -> Self {
        JsColor::new(JsValue::from_f64(value.into()))
    }
}
