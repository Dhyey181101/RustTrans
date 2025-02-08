
use std::f64::consts::PI;

struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

fn fast_contains(i: &GeoS1Interval, p: f64) -> bool {
    if is_inverted(i) {
        return (p >= i.lo || p <= i.hi) && !is_empty(i);
    }
    p >= i.lo && p <= i.hi
}

fn is_inverted(i: &GeoS1Interval) -> bool {
    i.lo > i.hi
}

fn is_empty(i: &GeoS1Interval) -> bool {
    i.lo == PI && i.hi == -PI
}
