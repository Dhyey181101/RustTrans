
use std::f64::consts::PI;

struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

fn interior_contains(i: &GeoS1Interval, p: f64) -> bool {
    let mut p = p;
    if p == -PI {
        p = PI;
    }
    if is_inverted(i) {
        return p > i.lo || p < i.hi;
    }
    (p > i.lo && p < i.hi) || is_full(i)
}

fn is_inverted(i: &GeoS1Interval) -> bool {
    i.lo > i.hi
}

fn is_full(i: &GeoS1Interval) -> bool {
    i.lo == -PI && i.hi == PI
}
