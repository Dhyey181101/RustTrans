
use std::f64::consts::PI;

struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

fn interior_contains(i: &GeoS1Interval, p: f64) -> bool {
    let p = if p == -PI { PI } else { p };
    if is_inverted(&i) {
        p > i.lo || p < i.hi
    } else {
        (p > i.lo && p < i.hi) || is_full(&i)
    }
}

fn is_inverted(i: &GeoS1Interval) -> bool {
    i.lo > i.hi
}

fn is_full(i: &GeoS1Interval) -> bool {
    i.lo == -PI && i.hi == PI  
}

