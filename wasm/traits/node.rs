use ordered_float::OrderedFloat;

pub trait Node
where Self: Clone + Copy + std::fmt::Debug + std::hash::Hash + Sized
{
    fn at_node(&self, other: &Self) -> bool;
    fn get_distance(&self, other: &Self) -> OrderedFloat<f64>;
    fn get_elevation(&self) -> i32;
    fn get_neighbors(&self) -> Vec<(Self, OrderedFloat<f64>)>;
    fn set_diagonal(&mut self, diagonal: bool);
}
