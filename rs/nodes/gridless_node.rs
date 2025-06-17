use crate::{
    traits::{JsDeserialize, JsHelper, JsSerialize, Node},
    types::{GridOffset2D, GridOffset3D},
};
use std::hash::Hash;

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Hash)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub struct GridlessNode {
    pub i: i32,
    pub j: i32,
    pub k: i32,
}

impl GridlessNode {
    pub fn new(i: i32, j: i32, k: i32) -> Self {
        Self { i, j, k }
    }
}

impl Node for GridlessNode {
    fn at_node(&self, other: &Self) -> bool {
        let _ = other;

        false
    }

    fn get_distance(&self, other: &Self) -> u32 {
        let _ = other;

        0
    }

    fn get_neighbors(&self) -> Vec<(Self, u32)> {
        Vec::new()
    }
}
