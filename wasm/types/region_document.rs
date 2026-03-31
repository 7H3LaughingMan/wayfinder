use crate::types::{Color, ElevatedPoint, foundry::documents::JsRegionDocument};
use geo::{BoundingRect, Contains, Coord, MultiPolygon, Rect, unary_union};
use std::ops::RangeInclusive;

#[derive(Clone, Debug)]
pub struct RegionDocument {
    pub id: String,
    color: Color,
    elevation: RangeInclusive<f64>,
    multi_polygon: MultiPolygon,
    bounds: Rect,
}

impl RegionDocument {
    pub fn color(&self) -> Color {
        self.color.clone()
    }

    pub fn multi_polygon(&self) -> MultiPolygon {
        self.multi_polygon.clone()
    }
}

impl From<JsRegionDocument> for RegionDocument {
    fn from(value: JsRegionDocument) -> Self {
        let multi_polygon = unary_union(&value.triangulation().to_polygons());
        let bounds = multi_polygon.bounding_rect().unwrap_or(Rect::new((0.0, 0.0), (0.0, 0.0)));

        RegionDocument {
            id: value.id(),
            color: value.color().into(),
            elevation: value.elevation().into(),
            multi_polygon,
            bounds,
        }
    }
}

impl BoundingRect<f64> for RegionDocument {
    type Output = Rect;

    fn bounding_rect(&self) -> Self::Output {
        self.bounds
    }
}

impl Contains<ElevatedPoint> for RegionDocument {
    fn contains(&self, ElevatedPoint { x, y, elevation }: &ElevatedPoint) -> bool {
        self.multi_polygon.contains(&Coord::from((*x, *y))) && self.elevation.contains(elevation)
    }
}

impl From<RegionDocument> for MultiPolygon {
    fn from(value: RegionDocument) -> Self {
        value.multi_polygon.clone()
    }
}
