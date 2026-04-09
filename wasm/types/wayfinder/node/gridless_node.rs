use crate::traits::Node;

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

impl Node for GridlessNode {}
