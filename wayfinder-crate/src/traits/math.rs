pub trait Math {
    fn clamp(self, min: Self, max: Self) -> Self;
    fn mix(a: Self, b: Self, w: Self) -> Self;
    fn between(self, a: Self, b: Self, inclusive: bool) -> bool;
}

impl Math for f32 {
    fn clamp(self, min: Self, max: Self) -> Self {
        f32::min(f32::max(self, min), max)
    }

    fn mix(a: Self, b: Self, w: Self) -> Self {
        a * (1.0 - w) + b * w
    }

    fn between(self, a: Self, b: Self, inclusive: bool) -> bool {
        let min = f32::min(a, b);
        let max = f32::max(a, b);

        if inclusive {
            self >= min && self <= max
        } else {
            self > min && self < max
        }
    }
}
