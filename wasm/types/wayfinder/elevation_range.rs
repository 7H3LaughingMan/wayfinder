use crate::types::{foundry::documents::JsElevationRange, wayfinder::ElevatedPoint};
use num_traits::{Num, NumCast};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug)]
pub struct ElevationRange<T = f64>
where T: Num + Clone + Copy + NumCast + PartialOrd + Debug
{
    bottom: T,
    top: T,
}

impl<T> ElevationRange<T>
where T: Num + Clone + Copy + NumCast + PartialOrd + Debug
{
    pub fn new<C>(bottom: C, top: C) -> Self
    where C: Into<T> {
        ElevationRange { bottom: bottom.into(), top: top.into() }
    }

    pub fn contains<C>(&self, elevation: C) -> bool
    where C: Into<T> {
        let elevation = elevation.into();
        self.bottom <= elevation && elevation <= self.top
    }

    pub fn intersects(&self, range: ElevationRange<T>) -> bool {
        !(self.bottom > range.top || self.top < range.bottom)
    }
}

impl ElevationRange {
    pub fn from_points(a: ElevatedPoint, b: ElevatedPoint) -> Self {
        ElevationRange { bottom: f64::min(a.elevation, b.elevation), top: f64::max(a.elevation, b.elevation) }
    }
}

impl From<JsElevationRange> for ElevationRange {
    fn from(value: JsElevationRange) -> Self {
        ElevationRange { bottom: value.bottom(), top: value.top() }
    }
}
