use crate::traits::Node;
use std::hash::Hash;

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Hash)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub struct SquareNode {
    pub i: i32,
    pub j: i32,
    pub d: bool,
}

impl SquareNode {
    pub fn new(i: i32, j: i32, d: bool) -> Self {
        Self { i, j, d }
    }

    pub fn from(&mut self, origin: &SquareNode) {
        let di = (origin.i - self.i).abs();
        let dj = (origin.j - self.j).abs();
        let nd = i32::min(di, dj) + origin.d as i32;

        self.d = (nd % 2) == 1;
    }
}

impl Node for SquareNode {
    fn at_node(&self, other: &Self) -> bool {
        self.i == other.i && self.j == other.j
    }

    fn get_distance(&self, other: &Self) -> u32 {
        let di = (self.i - other.i).unsigned_abs();
        let dj = (self.j - other.j).unsigned_abs();

        di + dj - ((u32::min(di, dj) + 1 - (self.d as u32)) >> 1)
    }

    fn get_neighbors(&self) -> Vec<(Self, u32)> {
        let SquareNode { i, j, d } = *self;

        vec![
            (SquareNode::new(i, j - 1, d), 1),
            (SquareNode::new(i + 1, j - 1, !d), if d { 2 } else { 1 }),
            (SquareNode::new(i + 1, j, d), 1),
            (SquareNode::new(i + 1, j + 1, !d), if d { 2 } else { 1 }),
            (SquareNode::new(i, j + 1, d), 1),
            (SquareNode::new(i - 1, j + 1, !d), if d { 2 } else { 1 }),
            (SquareNode::new(i - 1, j, d), 1),
            (SquareNode::new(i - 1, j - 1, !d), if d { 2 } else { 1 }),
        ]
    }
}
