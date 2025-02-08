
use std::f64::consts::PI;

struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

fn contains(i: &GeoS1Interval, p: f64) -> bool {
    let p = if p == -PI { PI } else { p };
    fast_contains(i, p)
}

fn fast_contains(i: &GeoS1Interval, p: f64) -> bool {
    if is_inverted(i) {
        (p >= i.lo || p <= i.hi) && !is_empty(i)
    } else {
        p >= i.lo && p <= i.hi 
    }
}

fn is_inverted(i: &GeoS1Interval) -> bool {
    i.lo > i.hi
}

fn is_empty(i: &GeoS1Interval) -> bool {
    i.lo == PI && i.hi == -PI
}
