pub fn between(num: f64, a: f64, b: f64, inclusive: Option<bool>) -> bool {
    let inclusive = inclusive.unwrap_or(true);
    let min = f64::min(a, b);
    let max = f64::max(a, b);
    if inclusive {
        return (num >= min) && (num <= max);
    } else {
        return (num > min) && (num < max);
    }
}
