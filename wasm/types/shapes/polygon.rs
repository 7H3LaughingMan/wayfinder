use crate::types::pixi::JsPolygon;
use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct Polygon {
    pub points: Vec<f64>,
}

impl Polygon {
    pub fn new(points: Vec<impl Into<f64>>) -> Self {
        Polygon { points: points.into_iter().map(Into::into).collect() }
    }

    pub fn contains(&self, x: impl Into<f64>, y: impl Into<f64>) -> bool {
        let x: f64 = x.into();
        let y: f64 = y.into();

        let mut inside = false;
        let length = self.points.len() / 2;

        let mut j = length - 1;
        for i in 0..length {
            let xi = self.points[i * 2];
            let yi = self.points[i * 2 + 1];
            let xj = self.points[j * 2];
            let yj = self.points[j * 2 + 1];

            if (yi > y) != (yj > y) && x < (xj - xi) * ((y - yi) / (yj - yi)) + xi {
                inside = !inside;
            }

            j = i;
        }

        inside
    }
}

impl From<JsPolygon> for Polygon {
    fn from(value: JsPolygon) -> Self {
        Polygon::new(value.points().iter().map(|n| n.value_of()).collect())
    }
}

impl From<geo::Polygon> for Polygon {
    fn from(value: geo::Polygon) -> Self {
        Polygon::new(value.exterior().coords().dropping_back(1).flat_map(|geo::Coord { x, y }| [*x, *y]).collect())
    }
}

impl From<Polygon> for geo::Polygon {
    fn from(value: Polygon) -> Self {
        geo::Polygon::new(value.points.iter().map(|n| *n).tuples::<(f64, f64)>().collect(), Vec::new())
    }
}
