use crate::types::{Color, ElevatedPoint, ElevationRange, foundry::documents::JsRegionDocument};
use geo::{BoundingRect, Intersects, Line, MultiPolygon, Rect, unary_union};

#[derive(Clone, Debug)]
pub struct RegionDocument {
    pub id: String,
    color: Color,
    elevation: ElevationRange,
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

impl Intersects<(ElevatedPoint, ElevatedPoint)> for RegionDocument {
    fn intersects(&self, (a, b): &(ElevatedPoint, ElevatedPoint)) -> bool {
        self.elevation.intersects(ElevationRange::from_points(*a, *b))
            && self.multi_polygon.intersects(&Line::new(*a, *b))
    }
}

impl From<RegionDocument> for MultiPolygon {
    fn from(value: RegionDocument) -> Self {
        value.multi_polygon.clone()
    }
}
