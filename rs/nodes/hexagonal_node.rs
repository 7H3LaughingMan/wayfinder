use crate::{
    grids::HexagonalGrid,
    traits::Node,
    types::{HexagonalGridCube2D, HexagonalGridCube3D},
};
use std::hash::Hash;

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Hash)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub struct HexagonalNode {
    pub q: i32,
    pub r: i32,
    pub s: i32,
    pub k: i32,
    pub d: bool,
}

impl HexagonalNode {
    pub fn new(q: i32, r: i32, s: i32, k: i32, d: bool) -> Self {
        Self { q, r, s, k, d }
    }
}

impl Node for HexagonalNode {
    fn at_node(&self, other: &Self) -> bool {
        self.q == other.q && self.r == other.r && self.s == other.s && self.k == other.k
    }

    fn get_distance(&self, other: &Self) -> u32 {
        let mut n = HexagonalGrid::cube_distance(
            HexagonalGridCube2D { q: self.q, r: self.r, s: self.s },
            HexagonalGridCube2D { q: other.q, r: other.r, s: other.s },
        );
        let mut d = (self.k - other.k).unsigned_abs();
        if n < d {
            (n, d) = (d, n);
        }

        n + ((d + if self.d { 1 } else { 0 }) / 2)
    }

    fn get_elevation(&self) -> i32 {
        self.k
    }

    fn get_neighbors(&self) -> Vec<(Self, u32)> {
        let HexagonalNode { q, r, s, k, d } = *self;

        vec![
            (HexagonalNode::new(q - 1, r, s + 1, k - 1, !d), if d { 2 } else { 1 }),
            (HexagonalNode::new(q - 1, r, s + 1, k, d), 1),
            (HexagonalNode::new(q - 1, r, s + 1, k + 1, !d), if d { 2 } else { 1 }),
            (HexagonalNode::new(q - 1, r + 1, s, k - 1, !d), if d { 2 } else { 1 }),
            (HexagonalNode::new(q - 1, r + 1, s, k, d), 1),
            (HexagonalNode::new(q - 1, r + 1, s, k + 1, !d), if d { 2 } else { 1 }),
            (HexagonalNode::new(q, r - 1, s + 1, k - 1, !d), if d { 2 } else { 1 }),
            (HexagonalNode::new(q, r - 1, s + 1, k, d), 1),
            (HexagonalNode::new(q, r - 1, s + 1, k + 1, !d), if d { 2 } else { 1 }),
            (HexagonalNode::new(q, r, s, k - 1, d), 1),
            (HexagonalNode::new(q, r, s, k + 1, d), 1),
            (HexagonalNode::new(q, r + 1, s - 1, k - 1, !d), if d { 2 } else { 1 }),
            (HexagonalNode::new(q, r + 1, s - 1, k, d), 1),
            (HexagonalNode::new(q, r + 1, s - 1, k + 1, !d), if d { 2 } else { 1 }),
            (HexagonalNode::new(q + 1, r - 1, s, k - 1, !d), if d { 2 } else { 1 }),
            (HexagonalNode::new(q + 1, r - 1, s, k, d), 1),
            (HexagonalNode::new(q + 1, r - 1, s, k + 1, !d), if d { 2 } else { 1 }),
            (HexagonalNode::new(q + 1, r, s - 1, k - 1, !d), if d { 2 } else { 1 }),
            (HexagonalNode::new(q + 1, r, s - 1, k, d), 1),
            (HexagonalNode::new(q + 1, r, s - 1, k + 1, !d), if d { 2 } else { 1 }),
        ]
    }

    fn set_diagonal(&mut self, diagonal: bool) {
        self.d = diagonal
    }
}
