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
    fn at_node(&self, _other: &Self) -> bool {
        false
    }

    fn get_distance(&self, _other: &Self) -> u32 {
        0
    }

    fn get_elevation(&self) -> i32 {
        self.k
    }

    fn get_neighbors(&self) -> Vec<(Self, u32)> {
        Vec::new()
    }

    fn set_diagonal(&mut self, _diagonal: bool) {
        todo!()
    }
}
