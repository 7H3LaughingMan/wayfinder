use crate::{
    GRID_DIAGONAL,
    traits::Node,
    types::{HexagonalGrid, foundry::GridDiagonalRule},
};
use ordered_float::OrderedFloat;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct GridlessNode {
    pub i: i32,
    pub j: i32,
    pub k: i32,
}

impl GridlessNode {
    pub fn new(i: impl Into<i32>, j: impl Into<i32>, k: impl Into<i32>) -> Self {
        Self { i: i.into(), j: j.into(), k: k.into() }
    }
}

impl Node for GridlessNode {
    fn at_node(&self, _other: &Self) -> bool {
        false
    }

    fn get_distance(&self, _other: &Self) -> OrderedFloat<f64> {
        OrderedFloat(0.0)
    }

    fn get_elevation(&self) -> i32 {
        self.k
    }

    fn get_neighbors(&self) -> Vec<(Self, OrderedFloat<f64>)> {
        Vec::new()
    }

    fn set_diagonal(&mut self, _diagonal: bool) {}
}

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

impl Node for SquareNode {
    fn at_node(&self, other: &Self) -> bool {
        self.i == other.i && self.j == other.j && self.k == other.k
    }

    fn get_distance(&self, other: &Self) -> OrderedFloat<f64> {
        let l0 = match *GRID_DIAGONAL {
            GridDiagonalRule::Alternating2 => {
                if self.d {
                    0.0
                } else {
                    1.0
                }
            }
            _ => {
                if self.d {
                    1.0
                } else {
                    0.0
                }
            }
        };
        let mut nd = l0 * 1.5;

        let mut di = (self.i - other.i).abs() as f64;
        let mut dj = (self.j - other.j).abs() as f64;
        if di < dj {
            (di, dj) = (dj, di);
        }
        let mut dk = (self.k - other.k).abs() as f64;
        if dj < dk {
            (dj, dk) = (dk, dj);
        }
        if di < dj {
            (di, dj) = (dj, di);
        }
        let nd0 = nd;

        (match *GRID_DIAGONAL {
            GridDiagonalRule::Equidistant => di,
            GridDiagonalRule::Exact => di + ((std::f64::consts::SQRT_2 - 1.0) * (dj - dk) + (crate::SQRT3 - 1.0) * dk),
            GridDiagonalRule::Approximate => di + 0.5 * (dj - dk) + 0.75 * dk,
            GridDiagonalRule::Rectilinear => di + (dj + dk),
            GridDiagonalRule::Alternating1 | GridDiagonalRule::Alternating2 => {
                nd += dj + 0.5 * dk;
                di + (nd / 2.0).floor() - (nd0 / 2.0).floor()
            }
            GridDiagonalRule::Illegal => di + (dj + dk),
        })
        .into()
    }

    fn get_elevation(&self) -> i32 {
        self.k
    }

    fn get_neighbors(&self) -> Vec<(Self, OrderedFloat<f64>)> {
        let SquareNode { i, j, k, d } = *self;

        match *GRID_DIAGONAL {
            GridDiagonalRule::Equidistant => vec![
                (SquareNode::new(i - 1, j - 1, k - 1, !d), OrderedFloat(1.0)),
                (SquareNode::new(i - 1, j - 1, k, !d), OrderedFloat(1.0)),
                (SquareNode::new(i - 1, j - 1, k + 1, !d), OrderedFloat(1.0)),
                (SquareNode::new(i - 1, j, k - 1, !d), OrderedFloat(1.0)),
                (SquareNode::new(i - 1, j, k, d), OrderedFloat(1.0)),
                (SquareNode::new(i - 1, j, k + 1, !d), OrderedFloat(1.0)),
                (SquareNode::new(i - 1, j + 1, k - 1, !d), OrderedFloat(1.0)),
                (SquareNode::new(i - 1, j + 1, k, !d), OrderedFloat(1.0)),
                (SquareNode::new(i - 1, j + 1, k + 1, !d), OrderedFloat(1.0)),
                (SquareNode::new(i, j - 1, k - 1, !d), OrderedFloat(1.0)),
                (SquareNode::new(i, j - 1, k, d), OrderedFloat(1.0)),
                (SquareNode::new(i, j - 1, k + 1, !d), OrderedFloat(1.0)),
                (SquareNode::new(i, j, k - 1, d), OrderedFloat(1.0)),
                (SquareNode::new(i, j, k + 1, d), OrderedFloat(1.0)),
                (SquareNode::new(i, j + 1, k - 1, !d), OrderedFloat(1.0)),
                (SquareNode::new(i, j + 1, k, d), OrderedFloat(1.0)),
                (SquareNode::new(i, j + 1, k + 1, !d), OrderedFloat(1.0)),
                (SquareNode::new(i + 1, j - 1, k - 1, !d), OrderedFloat(1.0)),
                (SquareNode::new(i + 1, j - 1, k, !d), OrderedFloat(1.0)),
                (SquareNode::new(i + 1, j - 1, k + 1, !d), OrderedFloat(1.0)),
                (SquareNode::new(i + 1, j, k - 1, !d), OrderedFloat(1.0)),
                (SquareNode::new(i + 1, j, k, d), OrderedFloat(1.0)),
                (SquareNode::new(i + 1, j, k + 1, !d), OrderedFloat(1.0)),
                (SquareNode::new(i + 1, j + 1, k - 1, !d), OrderedFloat(1.0)),
                (SquareNode::new(i + 1, j + 1, k, !d), OrderedFloat(1.0)),
                (SquareNode::new(i + 1, j + 1, k + 1, !d), OrderedFloat(1.0)),
            ],
            GridDiagonalRule::Exact => vec![
                (SquareNode::new(i - 1, j - 1, k - 1, !d), OrderedFloat(crate::SQRT3)),
                (SquareNode::new(i - 1, j - 1, k, !d), OrderedFloat(crate::SQRT2)),
                (SquareNode::new(i - 1, j - 1, k + 1, !d), OrderedFloat(crate::SQRT3)),
                (SquareNode::new(i - 1, j, k - 1, !d), OrderedFloat(crate::SQRT2)),
                (SquareNode::new(i - 1, j, k, d), OrderedFloat(1.0)),
                (SquareNode::new(i - 1, j, k + 1, !d), OrderedFloat(crate::SQRT2)),
                (SquareNode::new(i - 1, j + 1, k - 1, !d), OrderedFloat(crate::SQRT3)),
                (SquareNode::new(i - 1, j + 1, k, !d), OrderedFloat(crate::SQRT2)),
                (SquareNode::new(i - 1, j + 1, k + 1, !d), OrderedFloat(crate::SQRT3)),
                (SquareNode::new(i, j - 1, k - 1, !d), OrderedFloat(crate::SQRT2)),
                (SquareNode::new(i, j - 1, k, d), OrderedFloat(1.0)),
                (SquareNode::new(i, j - 1, k + 1, !d), OrderedFloat(crate::SQRT2)),
                (SquareNode::new(i, j, k - 1, d), OrderedFloat(1.0)),
                (SquareNode::new(i, j, k + 1, d), OrderedFloat(1.0)),
                (SquareNode::new(i, j + 1, k - 1, !d), OrderedFloat(crate::SQRT2)),
                (SquareNode::new(i, j + 1, k, d), OrderedFloat(1.0)),
                (SquareNode::new(i, j + 1, k + 1, !d), OrderedFloat(crate::SQRT2)),
                (SquareNode::new(i + 1, j - 1, k - 1, !d), OrderedFloat(crate::SQRT3)),
                (SquareNode::new(i + 1, j - 1, k, !d), OrderedFloat(crate::SQRT2)),
                (SquareNode::new(i + 1, j - 1, k + 1, !d), OrderedFloat(crate::SQRT3)),
                (SquareNode::new(i + 1, j, k - 1, !d), OrderedFloat(crate::SQRT2)),
                (SquareNode::new(i + 1, j, k, d), OrderedFloat(1.0)),
                (SquareNode::new(i + 1, j, k + 1, !d), OrderedFloat(crate::SQRT2)),
                (SquareNode::new(i + 1, j + 1, k - 1, !d), OrderedFloat(crate::SQRT3)),
                (SquareNode::new(i + 1, j + 1, k, !d), OrderedFloat(crate::SQRT2)),
                (SquareNode::new(i + 1, j + 1, k + 1, !d), OrderedFloat(crate::SQRT3)),
            ],
            GridDiagonalRule::Approximate => vec![
                (SquareNode::new(i - 1, j - 1, k - 1, !d), OrderedFloat(1.75)),
                (SquareNode::new(i - 1, j - 1, k, !d), OrderedFloat(1.5)),
                (SquareNode::new(i - 1, j - 1, k + 1, !d), OrderedFloat(1.75)),
                (SquareNode::new(i - 1, j, k - 1, !d), OrderedFloat(1.5)),
                (SquareNode::new(i - 1, j, k, d), OrderedFloat(1.0)),
                (SquareNode::new(i - 1, j, k + 1, !d), OrderedFloat(1.5)),
                (SquareNode::new(i - 1, j + 1, k - 1, !d), OrderedFloat(1.75)),
                (SquareNode::new(i - 1, j + 1, k, !d), OrderedFloat(1.5)),
                (SquareNode::new(i - 1, j + 1, k + 1, !d), OrderedFloat(1.75)),
                (SquareNode::new(i, j - 1, k - 1, !d), OrderedFloat(1.5)),
                (SquareNode::new(i, j - 1, k, d), OrderedFloat(1.0)),
                (SquareNode::new(i, j - 1, k + 1, !d), OrderedFloat(1.5)),
                (SquareNode::new(i, j, k - 1, d), OrderedFloat(1.0)),
                (SquareNode::new(i, j, k + 1, d), OrderedFloat(1.0)),
                (SquareNode::new(i, j + 1, k - 1, !d), OrderedFloat(1.5)),
                (SquareNode::new(i, j + 1, k, d), OrderedFloat(1.0)),
                (SquareNode::new(i, j + 1, k + 1, !d), OrderedFloat(1.5)),
                (SquareNode::new(i + 1, j - 1, k - 1, !d), OrderedFloat(1.75)),
                (SquareNode::new(i + 1, j - 1, k, !d), OrderedFloat(1.5)),
                (SquareNode::new(i + 1, j - 1, k + 1, !d), OrderedFloat(1.75)),
                (SquareNode::new(i + 1, j, k - 1, !d), OrderedFloat(1.5)),
                (SquareNode::new(i + 1, j, k, d), OrderedFloat(1.0)),
                (SquareNode::new(i + 1, j, k + 1, !d), OrderedFloat(1.5)),
                (SquareNode::new(i + 1, j + 1, k - 1, !d), OrderedFloat(1.75)),
                (SquareNode::new(i + 1, j + 1, k, !d), OrderedFloat(1.5)),
                (SquareNode::new(i + 1, j + 1, k + 1, !d), OrderedFloat(1.75)),
            ],
            GridDiagonalRule::Rectilinear => vec![
                (SquareNode::new(i - 1, j - 1, k - 1, !d), OrderedFloat(3.0)),
                (SquareNode::new(i - 1, j - 1, k, !d), OrderedFloat(2.0)),
                (SquareNode::new(i - 1, j - 1, k + 1, !d), OrderedFloat(3.0)),
                (SquareNode::new(i - 1, j, k - 1, !d), OrderedFloat(2.0)),
                (SquareNode::new(i - 1, j, k, d), OrderedFloat(1.0)),
                (SquareNode::new(i - 1, j, k + 1, !d), OrderedFloat(2.0)),
                (SquareNode::new(i - 1, j + 1, k - 1, !d), OrderedFloat(3.0)),
                (SquareNode::new(i - 1, j + 1, k, !d), OrderedFloat(2.0)),
                (SquareNode::new(i - 1, j + 1, k + 1, !d), OrderedFloat(3.0)),
                (SquareNode::new(i, j - 1, k - 1, !d), OrderedFloat(2.0)),
                (SquareNode::new(i, j - 1, k, d), OrderedFloat(1.0)),
                (SquareNode::new(i, j - 1, k + 1, !d), OrderedFloat(2.0)),
                (SquareNode::new(i, j, k - 1, d), OrderedFloat(1.0)),
                (SquareNode::new(i, j, k + 1, d), OrderedFloat(1.0)),
                (SquareNode::new(i, j + 1, k - 1, !d), OrderedFloat(2.0)),
                (SquareNode::new(i, j + 1, k, d), OrderedFloat(1.0)),
                (SquareNode::new(i, j + 1, k + 1, !d), OrderedFloat(2.0)),
                (SquareNode::new(i + 1, j - 1, k - 1, !d), OrderedFloat(3.0)),
                (SquareNode::new(i + 1, j - 1, k, !d), OrderedFloat(2.0)),
                (SquareNode::new(i + 1, j - 1, k + 1, !d), OrderedFloat(3.0)),
                (SquareNode::new(i + 1, j, k - 1, !d), OrderedFloat(2.0)),
                (SquareNode::new(i + 1, j, k, d), OrderedFloat(1.0)),
                (SquareNode::new(i + 1, j, k + 1, !d), OrderedFloat(2.0)),
                (SquareNode::new(i + 1, j + 1, k - 1, !d), OrderedFloat(3.0)),
                (SquareNode::new(i + 1, j + 1, k, !d), OrderedFloat(2.0)),
                (SquareNode::new(i + 1, j + 1, k + 1, !d), OrderedFloat(3.0)),
            ],
            GridDiagonalRule::Alternating1 => vec![
                (SquareNode::new(i - 1, j - 1, k - 1, !d), if d { OrderedFloat(2.0) } else { OrderedFloat(1.0) }),
                (SquareNode::new(i - 1, j - 1, k, !d), if d { OrderedFloat(2.0) } else { OrderedFloat(1.0) }),
                (SquareNode::new(i - 1, j - 1, k + 1, !d), if d { OrderedFloat(2.0) } else { OrderedFloat(1.0) }),
                (SquareNode::new(i - 1, j, k - 1, !d), if d { OrderedFloat(2.0) } else { OrderedFloat(1.0) }),
                (SquareNode::new(i - 1, j, k, d), OrderedFloat(1.0)),
                (SquareNode::new(i - 1, j, k + 1, !d), if d { OrderedFloat(2.0) } else { OrderedFloat(1.0) }),
                (SquareNode::new(i - 1, j + 1, k - 1, !d), if d { OrderedFloat(2.0) } else { OrderedFloat(1.0) }),
                (SquareNode::new(i - 1, j + 1, k, !d), if d { OrderedFloat(2.0) } else { OrderedFloat(1.0) }),
                (SquareNode::new(i - 1, j + 1, k + 1, !d), if d { OrderedFloat(2.0) } else { OrderedFloat(1.0) }),
                (SquareNode::new(i, j - 1, k - 1, !d), if d { OrderedFloat(2.0) } else { OrderedFloat(1.0) }),
                (SquareNode::new(i, j - 1, k, d), OrderedFloat(1.0)),
                (SquareNode::new(i, j - 1, k + 1, !d), if d { OrderedFloat(2.0) } else { OrderedFloat(1.0) }),
                (SquareNode::new(i, j, k - 1, d), OrderedFloat(1.0)),
                (SquareNode::new(i, j, k + 1, d), OrderedFloat(1.0)),
                (SquareNode::new(i, j + 1, k - 1, !d), if d { OrderedFloat(2.0) } else { OrderedFloat(1.0) }),
                (SquareNode::new(i, j + 1, k, d), OrderedFloat(1.0)),
                (SquareNode::new(i, j + 1, k + 1, !d), if d { OrderedFloat(2.0) } else { OrderedFloat(1.0) }),
                (SquareNode::new(i + 1, j - 1, k - 1, !d), if d { OrderedFloat(2.0) } else { OrderedFloat(1.0) }),
                (SquareNode::new(i + 1, j - 1, k, !d), if d { OrderedFloat(2.0) } else { OrderedFloat(1.0) }),
                (SquareNode::new(i + 1, j - 1, k + 1, !d), if d { OrderedFloat(2.0) } else { OrderedFloat(1.0) }),
                (SquareNode::new(i + 1, j, k - 1, !d), if d { OrderedFloat(2.0) } else { OrderedFloat(1.0) }),
                (SquareNode::new(i + 1, j, k, d), OrderedFloat(1.0)),
                (SquareNode::new(i + 1, j, k + 1, !d), if d { OrderedFloat(2.0) } else { OrderedFloat(1.0) }),
                (SquareNode::new(i + 1, j + 1, k - 1, !d), if d { OrderedFloat(2.0) } else { OrderedFloat(1.0) }),
                (SquareNode::new(i + 1, j + 1, k, !d), if d { OrderedFloat(2.0) } else { OrderedFloat(1.0) }),
                (SquareNode::new(i + 1, j + 1, k + 1, !d), if d { OrderedFloat(2.0) } else { OrderedFloat(1.0) }),
            ],
            GridDiagonalRule::Alternating2 => vec![
                (SquareNode::new(i - 1, j - 1, k - 1, !d), if d { OrderedFloat(1.0) } else { OrderedFloat(2.0) }),
                (SquareNode::new(i - 1, j - 1, k, !d), if d { OrderedFloat(1.0) } else { OrderedFloat(2.0) }),
                (SquareNode::new(i - 1, j - 1, k + 1, !d), if d { OrderedFloat(1.0) } else { OrderedFloat(2.0) }),
                (SquareNode::new(i - 1, j, k - 1, !d), if d { OrderedFloat(1.0) } else { OrderedFloat(2.0) }),
                (SquareNode::new(i - 1, j, k, d), OrderedFloat(1.0)),
                (SquareNode::new(i - 1, j, k + 1, !d), if d { OrderedFloat(1.0) } else { OrderedFloat(2.0) }),
                (SquareNode::new(i - 1, j + 1, k - 1, !d), if d { OrderedFloat(1.0) } else { OrderedFloat(2.0) }),
                (SquareNode::new(i - 1, j + 1, k, !d), if d { OrderedFloat(1.0) } else { OrderedFloat(2.0) }),
                (SquareNode::new(i - 1, j + 1, k + 1, !d), if d { OrderedFloat(1.0) } else { OrderedFloat(2.0) }),
                (SquareNode::new(i, j - 1, k - 1, !d), if d { OrderedFloat(1.0) } else { OrderedFloat(2.0) }),
                (SquareNode::new(i, j - 1, k, d), OrderedFloat(1.0)),
                (SquareNode::new(i, j - 1, k + 1, !d), if d { OrderedFloat(1.0) } else { OrderedFloat(2.0) }),
                (SquareNode::new(i, j, k - 1, d), OrderedFloat(1.0)),
                (SquareNode::new(i, j, k + 1, d), OrderedFloat(1.0)),
                (SquareNode::new(i, j + 1, k - 1, !d), if d { OrderedFloat(1.0) } else { OrderedFloat(2.0) }),
                (SquareNode::new(i, j + 1, k, d), OrderedFloat(1.0)),
                (SquareNode::new(i, j + 1, k + 1, !d), if d { OrderedFloat(1.0) } else { OrderedFloat(2.0) }),
                (SquareNode::new(i + 1, j - 1, k - 1, !d), if d { OrderedFloat(1.0) } else { OrderedFloat(2.0) }),
                (SquareNode::new(i + 1, j - 1, k, !d), if d { OrderedFloat(1.0) } else { OrderedFloat(2.0) }),
                (SquareNode::new(i + 1, j - 1, k + 1, !d), if d { OrderedFloat(1.0) } else { OrderedFloat(2.0) }),
                (SquareNode::new(i + 1, j, k - 1, !d), if d { OrderedFloat(1.0) } else { OrderedFloat(2.0) }),
                (SquareNode::new(i + 1, j, k, d), OrderedFloat(1.0)),
                (SquareNode::new(i + 1, j, k + 1, !d), if d { OrderedFloat(1.0) } else { OrderedFloat(2.0) }),
                (SquareNode::new(i + 1, j + 1, k - 1, !d), if d { OrderedFloat(1.0) } else { OrderedFloat(2.0) }),
                (SquareNode::new(i + 1, j + 1, k, !d), if d { OrderedFloat(1.0) } else { OrderedFloat(2.0) }),
                (SquareNode::new(i + 1, j + 1, k + 1, !d), if d { OrderedFloat(1.0) } else { OrderedFloat(2.0) }),
            ],
            GridDiagonalRule::Illegal => vec![
                (SquareNode::new(i - 1, j, k, d), OrderedFloat(1.0)),
                (SquareNode::new(i, j - 1, k, d), OrderedFloat(1.0)),
                (SquareNode::new(i, j, k - 1, d), OrderedFloat(1.0)),
                (SquareNode::new(i, j, k + 1, d), OrderedFloat(1.0)),
                (SquareNode::new(i, j + 1, k, d), OrderedFloat(1.0)),
                (SquareNode::new(i + 1, j, k, d), OrderedFloat(1.0)),
            ],
        }
    }

    fn set_diagonal(&mut self, diagonal: bool) {
        self.d = diagonal;
    }
}

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

impl Node for HexagonalNode {
    fn at_node(&self, other: &Self) -> bool {
        self.q == other.q && self.r == other.r && self.s == other.s && self.k == other.k
    }

    fn get_distance(&self, other: &Self) -> OrderedFloat<f64> {
        let mut nd = match *GRID_DIAGONAL {
            GridDiagonalRule::Alternating2 => {
                if self.d {
                    0.0
                } else {
                    1.0
                }
            }
            _ => {
                if self.d {
                    1.0
                } else {
                    0.0
                }
            }
        };

        let mut n = HexagonalGrid::cube_distance(*self, *other) as f64;
        let mut d = f64::abs((self.k as f64) - (other.k as f64));
        if n < d {
            (n, d) = (d, n);
        }
        let nd0 = nd;

        match *GRID_DIAGONAL {
            GridDiagonalRule::Equidistant => OrderedFloat(n),
            GridDiagonalRule::Exact => OrderedFloat(n + (crate::SQRT2 - 1.0) * d),
            GridDiagonalRule::Approximate => OrderedFloat(n + 0.5 * d),
            GridDiagonalRule::Rectilinear => OrderedFloat(n + d),
            GridDiagonalRule::Alternating1 | GridDiagonalRule::Alternating2 => {
                nd += d;
                OrderedFloat(n + (f64::floor(nd / 2.0) - f64::floor(nd0 / 2.0)))
            }
            GridDiagonalRule::Illegal => OrderedFloat(n + d),
        }
    }

    fn get_elevation(&self) -> i32 {
        self.k
    }

    fn get_neighbors(&self) -> Vec<(Self, OrderedFloat<f64>)> {
        let HexagonalNode { q, r, s, k, d } = *self;

        match *GRID_DIAGONAL {
            GridDiagonalRule::Equidistant => vec![
                (HexagonalNode::new(q - 1, r, s + 1, k - 1, !d), OrderedFloat(1.0)),
                (HexagonalNode::new(q - 1, r + 1, s, k - 1, !d), OrderedFloat(1.0)),
                (HexagonalNode::new(q, r - 1, s + 1, k - 1, !d), OrderedFloat(1.0)),
                (HexagonalNode::new(q, r + 1, s - 1, k - 1, !d), OrderedFloat(1.0)),
                (HexagonalNode::new(q + 1, r - 1, s, k - 1, !d), OrderedFloat(1.0)),
                (HexagonalNode::new(q + 1, r, s - 1, k - 1, !d), OrderedFloat(1.0)),
                (HexagonalNode::new(q - 1, r, s + 1, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q - 1, r + 1, s, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q, r - 1, s + 1, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q, r + 1, s - 1, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q + 1, r - 1, s, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q + 1, r, s - 1, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q - 1, r, s + 1, k + 1, !d), OrderedFloat(1.0)),
                (HexagonalNode::new(q - 1, r + 1, s, k + 1, !d), OrderedFloat(1.0)),
                (HexagonalNode::new(q, r - 1, s + 1, k + 1, !d), OrderedFloat(1.0)),
                (HexagonalNode::new(q, r + 1, s - 1, k + 1, !d), OrderedFloat(1.0)),
                (HexagonalNode::new(q + 1, r - 1, s, k + 1, !d), OrderedFloat(1.0)),
                (HexagonalNode::new(q + 1, r, s - 1, k + 1, !d), OrderedFloat(1.0)),
                (HexagonalNode::new(q, r, s, k - 1, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q, r, s, k + 1, d), OrderedFloat(1.0)),
            ],
            GridDiagonalRule::Exact => vec![
                (HexagonalNode::new(q - 1, r, s + 1, k - 1, !d), OrderedFloat(crate::SQRT2)),
                (HexagonalNode::new(q - 1, r + 1, s, k - 1, !d), OrderedFloat(crate::SQRT2)),
                (HexagonalNode::new(q, r - 1, s + 1, k - 1, !d), OrderedFloat(crate::SQRT2)),
                (HexagonalNode::new(q, r + 1, s - 1, k - 1, !d), OrderedFloat(crate::SQRT2)),
                (HexagonalNode::new(q + 1, r - 1, s, k - 1, !d), OrderedFloat(crate::SQRT2)),
                (HexagonalNode::new(q + 1, r, s - 1, k - 1, !d), OrderedFloat(crate::SQRT2)),
                (HexagonalNode::new(q - 1, r, s + 1, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q - 1, r + 1, s, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q, r - 1, s + 1, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q, r + 1, s - 1, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q + 1, r - 1, s, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q + 1, r, s - 1, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q - 1, r, s + 1, k + 1, !d), OrderedFloat(crate::SQRT2)),
                (HexagonalNode::new(q - 1, r + 1, s, k + 1, !d), OrderedFloat(crate::SQRT2)),
                (HexagonalNode::new(q, r - 1, s + 1, k + 1, !d), OrderedFloat(crate::SQRT2)),
                (HexagonalNode::new(q, r + 1, s - 1, k + 1, !d), OrderedFloat(crate::SQRT2)),
                (HexagonalNode::new(q + 1, r - 1, s, k + 1, !d), OrderedFloat(crate::SQRT2)),
                (HexagonalNode::new(q + 1, r, s - 1, k + 1, !d), OrderedFloat(crate::SQRT2)),
                (HexagonalNode::new(q, r, s, k - 1, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q, r, s, k + 1, d), OrderedFloat(1.0)),
            ],
            GridDiagonalRule::Approximate => vec![
                (HexagonalNode::new(q - 1, r, s + 1, k - 1, !d), OrderedFloat(1.5)),
                (HexagonalNode::new(q - 1, r + 1, s, k - 1, !d), OrderedFloat(1.5)),
                (HexagonalNode::new(q, r - 1, s + 1, k - 1, !d), OrderedFloat(1.5)),
                (HexagonalNode::new(q, r + 1, s - 1, k - 1, !d), OrderedFloat(1.5)),
                (HexagonalNode::new(q + 1, r - 1, s, k - 1, !d), OrderedFloat(1.5)),
                (HexagonalNode::new(q + 1, r, s - 1, k - 1, !d), OrderedFloat(1.5)),
                (HexagonalNode::new(q - 1, r, s + 1, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q - 1, r + 1, s, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q, r - 1, s + 1, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q, r + 1, s - 1, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q + 1, r - 1, s, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q + 1, r, s - 1, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q - 1, r, s + 1, k + 1, !d), OrderedFloat(1.5)),
                (HexagonalNode::new(q - 1, r + 1, s, k + 1, !d), OrderedFloat(1.5)),
                (HexagonalNode::new(q, r - 1, s + 1, k + 1, !d), OrderedFloat(1.5)),
                (HexagonalNode::new(q, r + 1, s - 1, k + 1, !d), OrderedFloat(1.5)),
                (HexagonalNode::new(q + 1, r - 1, s, k + 1, !d), OrderedFloat(1.5)),
                (HexagonalNode::new(q + 1, r, s - 1, k + 1, !d), OrderedFloat(1.5)),
                (HexagonalNode::new(q, r, s, k - 1, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q, r, s, k + 1, d), OrderedFloat(1.0)),
            ],
            GridDiagonalRule::Rectilinear => vec![
                (HexagonalNode::new(q - 1, r, s + 1, k - 1, !d), OrderedFloat(2.0)),
                (HexagonalNode::new(q - 1, r + 1, s, k - 1, !d), OrderedFloat(2.0)),
                (HexagonalNode::new(q, r - 1, s + 1, k - 1, !d), OrderedFloat(2.0)),
                (HexagonalNode::new(q, r + 1, s - 1, k - 1, !d), OrderedFloat(2.0)),
                (HexagonalNode::new(q + 1, r - 1, s, k - 1, !d), OrderedFloat(2.0)),
                (HexagonalNode::new(q + 1, r, s - 1, k - 1, !d), OrderedFloat(2.0)),
                (HexagonalNode::new(q - 1, r, s + 1, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q - 1, r + 1, s, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q, r - 1, s + 1, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q, r + 1, s - 1, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q + 1, r - 1, s, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q + 1, r, s - 1, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q - 1, r, s + 1, k + 1, !d), OrderedFloat(2.0)),
                (HexagonalNode::new(q - 1, r + 1, s, k + 1, !d), OrderedFloat(2.0)),
                (HexagonalNode::new(q, r - 1, s + 1, k + 1, !d), OrderedFloat(2.0)),
                (HexagonalNode::new(q, r + 1, s - 1, k + 1, !d), OrderedFloat(2.0)),
                (HexagonalNode::new(q + 1, r - 1, s, k + 1, !d), OrderedFloat(2.0)),
                (HexagonalNode::new(q + 1, r, s - 1, k + 1, !d), OrderedFloat(2.0)),
                (HexagonalNode::new(q, r, s, k - 1, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q, r, s, k + 1, d), OrderedFloat(1.0)),
            ],
            GridDiagonalRule::Alternating1 => vec![
                (HexagonalNode::new(q - 1, r, s + 1, k - 1, !d), if d { OrderedFloat(2.0) } else { OrderedFloat(1.0) }),
                (HexagonalNode::new(q - 1, r + 1, s, k - 1, !d), if d { OrderedFloat(2.0) } else { OrderedFloat(1.0) }),
                (HexagonalNode::new(q, r - 1, s + 1, k - 1, !d), if d { OrderedFloat(2.0) } else { OrderedFloat(1.0) }),
                (HexagonalNode::new(q, r + 1, s - 1, k - 1, !d), if d { OrderedFloat(2.0) } else { OrderedFloat(1.0) }),
                (HexagonalNode::new(q + 1, r - 1, s, k - 1, !d), if d { OrderedFloat(2.0) } else { OrderedFloat(1.0) }),
                (HexagonalNode::new(q + 1, r, s - 1, k - 1, !d), if d { OrderedFloat(2.0) } else { OrderedFloat(1.0) }),
                (HexagonalNode::new(q - 1, r, s + 1, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q - 1, r + 1, s, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q, r - 1, s + 1, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q, r + 1, s - 1, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q + 1, r - 1, s, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q + 1, r, s - 1, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q - 1, r, s + 1, k + 1, !d), if d { OrderedFloat(2.0) } else { OrderedFloat(1.0) }),
                (HexagonalNode::new(q - 1, r + 1, s, k + 1, !d), if d { OrderedFloat(2.0) } else { OrderedFloat(1.0) }),
                (HexagonalNode::new(q, r - 1, s + 1, k + 1, !d), if d { OrderedFloat(2.0) } else { OrderedFloat(1.0) }),
                (HexagonalNode::new(q, r + 1, s - 1, k + 1, !d), if d { OrderedFloat(2.0) } else { OrderedFloat(1.0) }),
                (HexagonalNode::new(q + 1, r - 1, s, k + 1, !d), if d { OrderedFloat(2.0) } else { OrderedFloat(1.0) }),
                (HexagonalNode::new(q + 1, r, s - 1, k + 1, !d), if d { OrderedFloat(2.0) } else { OrderedFloat(1.0) }),
                (HexagonalNode::new(q, r, s, k - 1, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q, r, s, k + 1, d), OrderedFloat(1.0)),
            ],
            GridDiagonalRule::Alternating2 => vec![
                (HexagonalNode::new(q - 1, r, s + 1, k - 1, !d), if d { OrderedFloat(1.0) } else { OrderedFloat(2.0) }),
                (HexagonalNode::new(q - 1, r + 1, s, k - 1, !d), if d { OrderedFloat(1.0) } else { OrderedFloat(2.0) }),
                (HexagonalNode::new(q, r - 1, s + 1, k - 1, !d), if d { OrderedFloat(1.0) } else { OrderedFloat(2.0) }),
                (HexagonalNode::new(q, r + 1, s - 1, k - 1, !d), if d { OrderedFloat(1.0) } else { OrderedFloat(2.0) }),
                (HexagonalNode::new(q + 1, r - 1, s, k - 1, !d), if d { OrderedFloat(1.0) } else { OrderedFloat(2.0) }),
                (HexagonalNode::new(q + 1, r, s - 1, k - 1, !d), if d { OrderedFloat(1.0) } else { OrderedFloat(2.0) }),
                (HexagonalNode::new(q - 1, r, s + 1, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q - 1, r + 1, s, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q, r - 1, s + 1, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q, r + 1, s - 1, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q + 1, r - 1, s, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q + 1, r, s - 1, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q - 1, r, s + 1, k + 1, !d), if d { OrderedFloat(1.0) } else { OrderedFloat(2.0) }),
                (HexagonalNode::new(q - 1, r + 1, s, k + 1, !d), if d { OrderedFloat(1.0) } else { OrderedFloat(2.0) }),
                (HexagonalNode::new(q, r - 1, s + 1, k + 1, !d), if d { OrderedFloat(1.0) } else { OrderedFloat(2.0) }),
                (HexagonalNode::new(q, r + 1, s - 1, k + 1, !d), if d { OrderedFloat(1.0) } else { OrderedFloat(2.0) }),
                (HexagonalNode::new(q + 1, r - 1, s, k + 1, !d), if d { OrderedFloat(1.0) } else { OrderedFloat(2.0) }),
                (HexagonalNode::new(q + 1, r, s - 1, k + 1, !d), if d { OrderedFloat(1.0) } else { OrderedFloat(2.0) }),
                (HexagonalNode::new(q, r, s, k - 1, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q, r, s, k + 1, d), OrderedFloat(1.0)),
            ],
            GridDiagonalRule::Illegal => vec![
                (HexagonalNode::new(q - 1, r, s + 1, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q - 1, r + 1, s, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q, r - 1, s + 1, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q, r + 1, s - 1, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q + 1, r - 1, s, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q + 1, r, s - 1, k, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q, r, s, k - 1, d), OrderedFloat(1.0)),
                (HexagonalNode::new(q, r, s, k + 1, d), OrderedFloat(1.0)),
            ],
        }
    }

    fn set_diagonal(&mut self, diagonal: bool) {
        self.d = diagonal;
    }
}
