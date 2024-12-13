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
    let epsilon = epsilon.unwrap_or(0.00000001);

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
