

use std::f64::consts::PI;

struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

fn intersects(i: Box<GeoS1Interval>, oi: Box<GeoS1Interval>) -> bool {
    if is_empty(i.as_ref()) || is_empty(oi.as_ref()) {
        return false;
    }
    if is_inverted(i.as_ref()) {
        return is_inverted(oi.as_ref()) || oi.lo <= i.hi || oi.hi >= i.lo;
    }
    if is_inverted(oi.as_ref()) {
        return oi.lo <= i.hi || oi.hi >= i.lo;
    }
    oi.lo <= i.hi && oi.hi >= i.lo
}

fn is_empty(i: &GeoS1Interval) -> bool {
    i.lo == PI && i.hi == -PI
}

fn is_inverted(i: &GeoS1Interval) -> bool {
    i.lo > i.hi
}

