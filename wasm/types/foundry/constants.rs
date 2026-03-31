use wasm_bindgen::prelude::*;

#[wasm_bindgen(skip_typescript)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub enum GridType {
    Gridless = 0,
    Square = 1,
    HexOddR = 2,
    HexEvenR = 3,
    HexOddQ = 4,
    HexEvenQ = 5,
}

#[wasm_bindgen(skip_typescript)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub enum GridDiagonalRule {
    Equidistant = 0,
    Exact = 1,
    Approximate = 2,
    Rectilinear = 3,
    Alternating1 = 4,
    Alternating2 = 5,
    Illegal = 6,
}

#[wasm_bindgen(skip_typescript)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub enum TokenShapeType {
    Ellipse1 = 0,
    Ellipse2 = 1,
    Trapezoid1 = 2,
    Trapezoid2 = 3,
    Rectangle1 = 4,
    Rectangle2 = 5,
}

impl Default for TokenShapeType {
    fn default() -> Self {
        TokenShapeType::Rectangle1
    }
}

#[wasm_bindgen(skip_typescript)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub enum WallDirection {
    Both = 0,
    Left = 1,
    Right = 2,
}

impl Default for WallDirection {
    fn default() -> Self {
        WallDirection::Both
    }
}

#[wasm_bindgen(skip_typescript)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub enum WallDoorType {
    None = 0,
    Door = 1,
    Secret = 2,
}

impl Default for WallDoorType {
    fn default() -> Self {
        WallDoorType::None
    }
}

#[wasm_bindgen(skip_typescript)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub enum WallDoorState {
    Closed = 0,
    Open = 1,
    Locked = 2,
}

impl Default for WallDoorState {
    fn default() -> Self {
        WallDoorState::Closed
    }
}

#[wasm_bindgen(skip_typescript)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub enum WallSenseType {
    None = 0,
    Limited = 10,
    Normal = 20,
    Proximity = 30,
    Distance = 40,
}

impl Default for WallSenseType {
    fn default() -> Self {
        WallSenseType::Normal
    }
}

#[wasm_bindgen(skip_typescript)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub enum WallMovementType {
    None = 0,
    Normal = 20,
}

impl Default for WallMovementType {
    fn default() -> Self {
        WallMovementType::Normal
    }
}
