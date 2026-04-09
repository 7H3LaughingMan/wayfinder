use crate::types::wayfinder::Point;

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
