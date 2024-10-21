use rapier2d::{na::Point2, prelude::Polyline};

use crate::types::Point;

#[derive(Clone)]
#[derive(Debug)]
pub struct TokenShape {
    pub center: Point,
    pub offset: Point,
    pub points: Vec<Point>,
    pub polyline: Polyline,
}

impl TokenShape {
    pub fn new(center: Point, offset: Point, points: Vec<Point>) -> Self {
        TokenShape { center, offset, points: points.clone(), polyline: TokenShape::get_polyline(points.clone()) }
    }

    pub fn centroid(points: &Vec<Point>) -> Point {
        if points.is_empty() {
            return Point { x: 0_f32, y: 0_f32 };
        }

        let mut x = 0_f32;
        let mut y = 0_f32;
        let mut a = 0_f32;

        let Point { x: mut x0, y: mut y0 } = points[points.len() - 1];

        for Point { x: x1, y: y1 } in points {
            let z = (x0 * y1) - (x1 * y0);
            x += (x0 + x1) * z;
            y += (y0 + y1) * z;
            x0 = *x1;
            y0 = *y1;
            a += z;
        }

        a *= 3.0;
        x /= a;
        y /= a;

        Point { x, y }
    }

    pub fn shrink(&self, size: Point) -> TokenShape {
        let mut points = Vec::new();

        for Point { x, y } in self.points.clone() {
            points.push(Point {
                x: if x < 0.0 {
                    x + size.x
                } else if x > 0.0 {
                    x - size.x
                } else {
                    0.0
                },
                y: if y < 0.0 {
                    y + size.y
                } else if x > 0.0 {
                    y - size.y
                } else {
                    0.0
                },
            });
        }

        TokenShape::new(self.center, self.offset, points)
    }

    fn get_vertices(points: Vec<Point>) -> Vec<Point2<f32>> {
        let mut vertices = Vec::new();

        for Point { x, y } in points {
            vertices.push(Point2::<f32>::new(x, y));
        }

        vertices
    }

    fn get_indices(points: Vec<Point>) -> Vec<[u32; 2]> {
        let mut indices = Vec::new();

        for i in 0..(points.len() as u32 - 1) {
            indices.push([i, i + 1]);
        }
        indices.push([points.len() as u32 - 1, 0]);

        indices
    }

    fn get_polyline(points: Vec<Point>) -> Polyline {
        let vertices = TokenShape::get_vertices(points.clone());
        let indices = Some(TokenShape::get_indices(points.clone()));

        Polyline::new(vertices, indices)
    }
}
