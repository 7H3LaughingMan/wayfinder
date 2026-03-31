#![allow(dead_code)]

use wasm_bindgen::prelude::*;

mod exports;
mod modules;
mod traits;
mod types;

const CLIPPER_SCALING_FACTOR: f64 = 100.0;

const EPSILON: f64 = 0.00000001;
/// π = 3.141592653589793
const PI: f64 = 3.141592653589793;
/// √2 = 1.4142135623730951
const SQRT2: f64 = 1.4142135623730951;
/// √½ = 0.7071067811865476
const SQRT1_2: f64 = 0.7071067811865476;
/// √3 = 1.7320508075688772
const SQRT3: f64 = 1.7320508075688772;
/// √⅓ = 0.5773502691896257
const SQRT1_3: f64 = 0.5773502691896257;

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        web_sys::console::log_1(&format!($($arg)*).into())
    };
}

#[macro_export]
macro_rules! push_str {
    ($dst:expr, $($arg:tt)*) => {
        $dst.push_str(&format!($($arg)*))
    };
}

#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();
}

static CANVAS: std::sync::LazyLock<types::foundry::canvas::JsCanvas> = std::sync::LazyLock::new(|| {
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(thread_local_v2, js_name = canvas)]
        static CANVAS: types::foundry::canvas::JsCanvas;
    }

    CANVAS.with(types::foundry::canvas::JsCanvas::clone)
});

static CONFIG: std::sync::LazyLock<types::foundry::config::JsConfig> = std::sync::LazyLock::new(|| {
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(thread_local_v2, js_name = CONFIG)]
        static CONFIG: types::foundry::config::JsConfig;
    }

    CONFIG.with(types::foundry::config::JsConfig::clone)
});

static GAME: std::sync::LazyLock<types::foundry::JsGame> = std::sync::LazyLock::new(|| {
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(thread_local_v2, js_name = game)]
        static GAME: types::foundry::JsGame;
    }

    GAME.with(types::foundry::JsGame::clone)
});

static GRID_DIAGONAL: std::sync::LazyLock<types::foundry::GridDiagonalRule> =
    std::sync::LazyLock::new(|| GAME.settings().get_setting("core", "gridDiagonals"));
