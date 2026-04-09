use crate::{
    traits::Node,
    types::wayfinder::{HexagonalGridCube2D, HexagonalGridCube3D},
};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct HexagonalNode {
    pub q: i32,
    pub r: i32,
    pub s: i32,
    pub k: i32,
    pub d: bool,
}

impl HexagonalNode {
    pub fn new(q: impl Into<i32>, r: impl Into<i32>, s: impl Into<i32>, k: impl Into<i32>, d: impl Into<bool>) -> Self {
        Self { q: q.into(), r: r.into(), s: s.into(), k: k.into(), d: d.into() }
    }
}

impl From<HexagonalGridCube2D> for HexagonalNode {
    fn from(HexagonalGridCube2D { q, r, s }: HexagonalGridCube2D) -> Self {
        HexagonalNode { q, r, s, k: 0, d: false }
    }
}

impl From<HexagonalGridCube3D> for HexagonalNode {
    fn from(HexagonalGridCube3D { q, r, s, k }: HexagonalGridCube3D) -> Self {
        HexagonalNode { q, r, s, k, d: false }
    }
}

impl Node for HexagonalNode {}
