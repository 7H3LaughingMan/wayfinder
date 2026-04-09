pub fn mix(a: impl Into<f64>, b: impl Into<f64>, w: f64) -> f64 {
    a.into() * (1.0 - w) + b.into() * w
}
