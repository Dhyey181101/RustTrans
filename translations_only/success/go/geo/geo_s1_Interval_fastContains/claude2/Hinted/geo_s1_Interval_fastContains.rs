
use std::f64::consts::PI;

struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

fn is_inverted(interval: &GeoS1Interval) -> bool {
    interval.lo > interval.hi
}

fn is_empty(interval: &GeoS1Interval) -> bool {
    interval.lo == PI && interval.hi == -PI 
}

fn fast_contains(interval: &GeoS1Interval, p: f64) -> bool {
    if is_inverted(interval) {
        (p >= interval.lo || p <= interval.hi) && !is_empty(interval)
    } else {
        p >= interval.lo && p <= interval.hi
    }
}
