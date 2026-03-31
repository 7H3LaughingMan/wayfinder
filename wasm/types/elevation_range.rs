use crate::types::{ElevatedPoint, foundry::documents::JsElevationRange};

#[derive(Clone, Copy, Debug)]
pub struct ElevationRange {
    bottom: f64,
    top: f64,
}

impl ElevationRange {
    pub fn contains(&self, elevation: f64) -> bool {
        self.bottom <= elevation && elevation <= self.top
    }

    pub fn intersects(&self, range: ElevationRange) -> bool {
        !(self.bottom > range.top || self.top < range.bottom)
    }

    pub fn from_points(a: ElevatedPoint, b: ElevatedPoint) -> Self {
        ElevationRange { bottom: f64::min(a.elevation, b.elevation), top: f64::max(a.elevation, b.elevation) }
    }
}

impl From<JsElevationRange> for ElevationRange {
    fn from(value: JsElevationRange) -> Self {
        ElevationRange { bottom: value.bottom(), top: value.top() }
    }
}
