
use std::f64::consts::PI;

struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

fn is_inverted(i: &GeoS1Interval) -> bool {
    i.lo > i.hi
}

fn is_empty(i: &GeoS1Interval) -> bool {
    i.lo == PI && i.hi == -PI
}

fn is_full(i: &GeoS1Interval) -> bool {
    i.lo == -PI && i.hi == PI
}

fn interior_contains_interval(i: &GeoS1Interval, oi: &GeoS1Interval) -> bool {
    if is_inverted(i) {
        if is_inverted(oi) {
            return (oi.lo > i.lo && oi.hi < i.hi) || is_empty(oi);
        }
        return oi.lo > i.lo || oi.hi < i.hi;
    }
    if is_inverted(oi) {
        return is_full(i) || is_empty(oi);
    }
    (oi.lo > i.lo && oi.hi < i.hi) || is_full(i)
}
