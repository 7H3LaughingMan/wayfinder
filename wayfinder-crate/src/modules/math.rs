use core::f64;

pub fn clamp(num: f64, min: f64, max: f64) -> f64 {
    f64::min(f64::max(num, min), max)
}

pub fn normalize_radians(radians: f64) -> f64 {
    let pi = f64::consts::PI;
    let pi2 = pi * 2.0;
    return radians - (pi2 * f64::floor((radians + pi) / pi2));
}
