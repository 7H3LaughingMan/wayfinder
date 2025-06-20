use crate::{
    traits::{JsDeserialize, JsHelper, JsSerialize, Node},
    types::{GridOffset2D, GridOffset3D},
};

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Hash)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub struct SquareNode {
    pub i: i32,
    pub j: i32,
    pub k: i32,
    pub d: bool,
}

impl SquareNode {
    pub fn new(i: i32, j: i32, k: i32, d: bool) -> Self {
        Self { i, j, k, d }
    }

    #[allow(unused_assignments)]
    pub fn from(&mut self, origin: &SquareNode) {
        let mut di = (self.i - origin.i).unsigned_abs();
        let mut dj = (self.j - origin.j).unsigned_abs();
        if di < dj {
            (di, dj) = (dj, di);
        }
        let mut dk = (self.k - origin.k).unsigned_abs();
        if dj < dk {
            (dj, dk) = (dk, dj);
        }
        if di < dj {
            (di, dj) = (dj, di);
        }

        let nd = dj as f64 + (0.5 * dk as f64) + if origin.d { 1.0 } else { 0.0 };
        let nd = (nd / 2.0).floor() as u32;
        self.d = (nd % 2) == 1;
    }
}

impl Node for SquareNode {
    fn at_node(&self, other: &Self) -> bool {
        self.i == other.i && self.j == other.j && self.k == other.k
    }

    fn get_distance(&self, other: &Self) -> u32 {
        let mut di = (self.i - other.i).unsigned_abs();
        let mut dj = (self.j - other.j).unsigned_abs();
        if di < dj {
            (di, dj) = (dj, di);
        }
        let mut dk = (self.k - other.k).unsigned_abs();
        if dj < dk {
            (dj, dk) = (dk, dj);
        }
        if di < dj {
            (di, dj) = (dj, di);
        }

        let nd = dj as f64 + (0.5 * dk as f64) + if self.d { 1.0 } else { 0.0 };
        let nd = (nd / 2.0).floor() as u32;
        di + nd
    }

    fn get_elevation(&self) -> i32 {
        self.k
    }

    fn get_neighbors(&self) -> Vec<(Self, u32)> {
        let SquareNode { i, j, k, d } = *self;

        vec![
            (SquareNode::new(i - 1, j - 1, k - 1, !d), if d { 2 } else { 1 }),
            (SquareNode::new(i - 1, j - 1, k, !d), if d { 2 } else { 1 }),
            (SquareNode::new(i - 1, j - 1, k + 1, !d), if d { 2 } else { 1 }),
            (SquareNode::new(i - 1, j, k - 1, !d), if d { 2 } else { 1 }),
            (SquareNode::new(i - 1, j, k, d), 1),
            (SquareNode::new(i - 1, j, k + 1, !d), if d { 2 } else { 1 }),
            (SquareNode::new(i - 1, j + 1, k - 1, !d), if d { 2 } else { 1 }),
            (SquareNode::new(i - 1, j + 1, k, !d), if d { 2 } else { 1 }),
            (SquareNode::new(i - 1, j + 1, k + 1, !d), if d { 2 } else { 1 }),
            (SquareNode::new(i, j - 1, k - 1, !d), if d { 2 } else { 1 }),
            (SquareNode::new(i, j - 1, k, d), 1),
            (SquareNode::new(i, j - 1, k + 1, !d), if d { 2 } else { 1 }),
            (SquareNode::new(i, j, k - 1, d), 1),
            (SquareNode::new(i, j, k + 1, d), 1),
            (SquareNode::new(i, j + 1, k - 1, !d), if d { 2 } else { 1 }),
            (SquareNode::new(i, j + 1, k, d), 1),
            (SquareNode::new(i, j + 1, k + 1, !d), if d { 2 } else { 1 }),
            (SquareNode::new(i + 1, j - 1, k - 1, !d), if d { 2 } else { 1 }),
            (SquareNode::new(i + 1, j - 1, k, !d), if d { 2 } else { 1 }),
            (SquareNode::new(i + 1, j - 1, k + 1, !d), if d { 2 } else { 1 }),
            (SquareNode::new(i + 1, j, k - 1, !d), if d { 2 } else { 1 }),
            (SquareNode::new(i + 1, j, k, d), 1),
            (SquareNode::new(i + 1, j, k + 1, !d), if d { 2 } else { 1 }),
            (SquareNode::new(i + 1, j + 1, k - 1, !d), if d { 2 } else { 1 }),
            (SquareNode::new(i + 1, j + 1, k, !d), if d { 2 } else { 1 }),
            (SquareNode::new(i + 1, j + 1, k + 1, !d), if d { 2 } else { 1 }),
        ]
    }

    fn set_diagonal(&mut self, diagonal: bool) {
        self.d = diagonal
    }
}
