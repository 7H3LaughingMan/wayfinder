use std::cmp::Ordering;

pub trait Match<T> {
    fn from_match(value: T) -> Self;
    fn to_match(value: Self) -> T;
}

impl Match<i8> for Ordering {
    fn from_match(value: i8) -> Self {
        match value {
            -1 => Ordering::Less,
            0 => Ordering::Equal,
            1 => Ordering::Greater,
            value_ => panic!("Unknown Ordering Value - {value_}!"),
        }
    }

    fn to_match(value: Self) -> i8 {
        match value {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }
}
