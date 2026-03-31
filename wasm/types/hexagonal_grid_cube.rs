use crate::types::HexagonalNode;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct HexagonalGridCube2D {
    pub q: i32,
    pub r: i32,
    pub s: i32,
}

impl From<HexagonalGridCube3D> for HexagonalGridCube2D {
    fn from(HexagonalGridCube3D { q, r, s, k: _ }: HexagonalGridCube3D) -> Self {
        HexagonalGridCube2D { q, r, s }
    }
}

impl From<HexagonalNode> for HexagonalGridCube2D {
    fn from(HexagonalNode { q, r, s, k: _, d: _ }: HexagonalNode) -> Self {
        HexagonalGridCube2D { q, r, s }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct HexagonalGridCube3D {
    pub q: i32,
    pub r: i32,
    pub s: i32,
    pub k: i32,
}

impl From<HexagonalGridCube2D> for HexagonalGridCube3D {
    fn from(HexagonalGridCube2D { q, r, s }: HexagonalGridCube2D) -> Self {
        HexagonalGridCube3D { q, r, s, k: 0 }
    }
}

impl From<HexagonalNode> for HexagonalGridCube3D {
    fn from(HexagonalNode { q, r, s, k, d: _ }: HexagonalNode) -> Self {
        HexagonalGridCube3D { q, r, s, k }
    }
}
