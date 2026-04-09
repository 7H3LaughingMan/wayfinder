use crate::{
    traits::Node,
    types::wayfinder::{GridOffset2D, GridOffset3D},
};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SquareNode {
    pub i: i32,
    pub j: i32,
    pub k: i32,
    pub d: bool,
}

impl SquareNode {
    pub fn new(i: impl Into<i32>, j: impl Into<i32>, k: impl Into<i32>, d: impl Into<bool>) -> Self {
        Self { i: i.into(), j: j.into(), k: k.into(), d: d.into() }
    }
}

impl From<GridOffset2D> for SquareNode {
    fn from(GridOffset2D { i, j }: GridOffset2D) -> Self {
        SquareNode { i, j, k: 0, d: false }
    }
}

impl From<GridOffset3D> for SquareNode {
    fn from(GridOffset3D { i, j, k }: GridOffset3D) -> Self {
        SquareNode { i, j, k, d: false }
    }
}

impl Node for SquareNode {}
