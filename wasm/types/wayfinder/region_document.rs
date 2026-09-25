use crate::{
    log,
    types::{
        foundry::documents::JsRegionDocument,
        wayfinder::{Color, ElevatedPoint, ElevationRange},
    },
};
use geo::{BoundingRect, Intersects, Line, MultiPolygon, Rect, unary_union};
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct RegionDocument {
    pub id: String,
    color: Color,
    elevation: ElevationRange,
    levels: HashSet<String>,
    multi_polygon: MultiPolygon,
    bounds: Rect,
}

impl RegionDocument {
    pub fn new(
        id: String,
        color: Color,
        elevation: ElevationRange,
        levels: HashSet<String>,
        multi_polygon: MultiPolygon,
    ) -> Self {
        let bounds = multi_polygon.bounding_rect().unwrap_or(Rect::new((0.0, 0.0), (0.0, 0.0)));

        RegionDocument { id, color, elevation, levels, multi_polygon, bounds }
    }

    pub fn color(&self) -> Color {
        self.color.clone()
    }

    pub fn multi_polygon(&self) -> MultiPolygon {
        self.multi_polygon.clone()
    }
}

impl From<JsRegionDocument> for RegionDocument {
    fn from(value: JsRegionDocument) -> Self {
        let behaviors = value.behaviors();

        for behavior in behaviors.values().into_iter().flatten() {
            log!(
                "id - {}, name - {}, type - {}, disabled - {}",
                behavior.id(),
                behavior.name(),
                behavior.r#type(),
                behavior.disabled()
            );
        }

        RegionDocument::new(
            value.id(),
            value.color().into(),
            value.elevation().into(),
            value.levels().values().into_iter().flatten().map(String::from).collect(),
            unary_union(&value.triangulation().to_polygons()),
        )
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
