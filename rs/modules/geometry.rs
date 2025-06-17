use crate::{
    modules::{math, number},
    types::{LineIntersection, Point},
};

pub fn orient_2d_fast(a: &Point, b: &Point, c: &Point) -> f64 {
    (a.y - c.y) * (b.x - c.x) - (a.x - c.x) * (b.y - c.y)
}

pub fn line_segement_intersects(a: &Point, b: &Point, c: &Point, d: &Point) -> bool {
    let xa = orient_2d_fast(a, b, c);
    let xb = orient_2d_fast(a, b, d);

    if xa != 0.0 && xb != 0.0 {
        return false;
    }
    let xab = (xa * xb) <= 0.0;

    let xcd = orient_2d_fast(c, d, a) * orient_2d_fast(c, d, b) <= 0.0;
    return xab && xcd;
}

pub fn line_line_intersection(a: &Point, b: &Point, c: &Point, d: &Point) -> Option<LineIntersection> {
    if ((a.x == b.x) && (a.y == b.y)) || ((c.x == d.x) && (c.y == d.y)) {
        return None;
    }

    let dnm = (d.y - c.y) * (b.x - a.x) - (d.x - c.x) * (b.y - a.y);
    if dnm == 0.0 {
        return None;
    }

    let t0 = ((d.x - c.x) * (a.y - c.y) - (d.y - c.y) * (a.x - c.x)) / dnm;
    let t1 = ((b.x - a.x) * (a.y - c.y) - (b.y - a.y) * (a.x - c.x)) / dnm;

    return Some(LineIntersection { x: a.x + t0 * (b.x - a.x), y: a.y + t0 * (b.y - a.y), t0, t1 });
}

pub fn line_segment_intersection(
    a: &Point,
    b: &Point,
    c: &Point,
    d: &Point,
    epsilon: Option<f64>,
) -> Option<LineIntersection> {
    let epsilon = epsilon.unwrap_or(1e-8);

    if ((a.x == b.x) && (a.y == b.y)) || ((c.x == d.x) && (c.y == d.y)) {
        return None;
    }

    let dnm = (d.y - c.y) * (b.x - a.x) - (d.x - c.x) * (b.y - a.y);
    if dnm == 0.0 {
        return None;
    }

    let t0 = ((d.x - c.x) * (a.y - c.y) - (d.y - c.y) * (a.x - c.x)) / dnm;
    if !number::between(t0, 0.0 - epsilon, 1.0 + epsilon, None) {
        return None;
    }

    let t1 = ((b.x - a.x) * (a.y - c.y) - (b.y - a.y) * (a.x - c.x)) / dnm;
    if !number::between(t1, 0.0 - epsilon, 1.0 + epsilon, None) {
        return None;
    }

    return Some(LineIntersection {
        x: a.x + t0 * (b.x - a.x),
        y: a.y + t0 * (b.y - a.y),
        t0: math::clamp(t0, 0.0, 1.0),
        t1: math::clamp(t1, 0.0, 1.0),
    });
}

pub fn closest_point_to_segment(c: &Point, a: &Point, b: &Point) -> Option<Point> {
    let dx = b.x - a.x;
    let dy = b.y - a.y;

    if dx == 0.0 && dy == 0.0 {
        return None;
    }

    let u = (((c.x - a.x) * dx) + ((c.y - a.y) * dy)) / (dx * dx + dy * dy);
    if u < 0.0 {
        Some(*a)
    } else if u > 1.0 {
        Some(*b)
    } else {
        Some(Point { x: a.x + (u * dx), y: a.y + (u * dy) })
    }
}

pub fn polygon_centroid(points: &Vec<Point>) -> Point {
    let n = points.len();

    if n == 0 {
        return Point { x: 0.0, y: 0.0 };
    }

    let mut x = 0.0;
    let mut y = 0.0;
    let mut a = 0.0;

    let Point { x: mut x0, y: mut y0 } = points[n - 1];
    for i in 0..n {
        let Point { x: x1, y: y1 } = points[i];
        let z = (x0 * y1) - (x1 * y0);
        x += (x0 + x1) * z;
        y += (y0 + y1) * z;
        x0 = x1;
        y0 = y1;
        a += z;
    }

    a *= 3.0;
    x /= a;
    y /= a;

    Point { x, y }
}
