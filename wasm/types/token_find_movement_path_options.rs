use js_sys::Array;
use wasm_bindgen::JsCast;

use crate::types::{
    TokenMovementWaypoint,
    foundry::{JsTokenFindMovementPathOptions, documents::JsTokenMovementWaypoint},
};

#[derive(Clone, Debug)]
pub struct TokenFindMovementPathOptions {
    pub preview: bool,
    pub ignore_walls: bool,
    pub ignore_cost: bool,
    pub history: bool,
    pub waypoints: Vec<TokenMovementWaypoint>,
    pub delay: f64,
}

impl Default for TokenFindMovementPathOptions {
    fn default() -> Self {
        Self {
            preview: false,
            ignore_walls: false,
            ignore_cost: false,
            history: false,
            waypoints: Vec::new(),
            delay: 0.0,
        }
    }
}

impl From<Option<JsTokenFindMovementPathOptions>> for TokenFindMovementPathOptions {
    fn from(value: Option<JsTokenFindMovementPathOptions>) -> Self {
        match value {
            Some(value) => value.into(),
            None => TokenFindMovementPathOptions::default(),
        }
    }
}

impl From<JsTokenFindMovementPathOptions> for TokenFindMovementPathOptions {
    fn from(value: JsTokenFindMovementPathOptions) -> Self {
        let mut waypoints = Vec::new();
        let history = match value.history() {
            Some(value) => match Array::is_array(&value) {
                true => {
                    waypoints = value
                        .unchecked_ref::<Array<JsTokenMovementWaypoint>>()
                        .iter()
                        .map(TokenMovementWaypoint::from)
                        .collect();
                    true
                }
                false => value.as_bool().unwrap_or(false),
            },
            None => false,
        };

        TokenFindMovementPathOptions {
            preview: value.preview().unwrap_or(false),
            ignore_walls: value.ignore_walls().unwrap_or(false),
            ignore_cost: value.ignore_cost().unwrap_or(false),
            history,
            waypoints,
            delay: value.delay().unwrap_or(0.0),
        }
    }
}
