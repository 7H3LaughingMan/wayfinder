pub trait Node
where
    Self: Sized,
{
    fn at_node(&self, other: &Self) -> bool;
    fn get_distance(&self, other: &Self) -> u32;
    fn get_neighbors(&self) -> Vec<(Self, u32)>;
}
