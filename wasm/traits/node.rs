use std::{fmt::Debug, hash::Hash};

pub trait Node
where Self: Clone + Copy + Debug + Hash + Sized
{
}
