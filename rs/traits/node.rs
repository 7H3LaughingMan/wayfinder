use std::fmt::Debug;
use std::hash::Hash;

pub trait Node
where Self: Clone + Copy + Debug + Hash + Sized
{
    fn at_node(&self, other: &Self) -> bool;
    fn get_distance(&self, other: &Self) -> u32;
    fn get_neighbors(&self) -> Vec<(Self, u32)>;
}
