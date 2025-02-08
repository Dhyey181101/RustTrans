

use std::f64::consts::PI;

struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

fn intersects(i: Box<GeoS1Interval>, oi: Box<GeoS1Interval>) -> bool {
    if is_empty(&i) || is_empty(&oi) {
        return false;
    }
    if is_inverted(&i) {
        return is_inverted(&oi) || oi.lo <= i.hi || oi.hi >= i.lo;
    }
    if is_inverted(&oi) {
        return oi.lo <= i.hi || oi.hi >= i.lo;
    }
    oi.lo <= i.hi && oi.hi >= i.lo
}

fn is_empty(i: &Box<GeoS1Interval>) -> bool {
    i.lo == PI && i.hi == -PI
}

fn is_inverted(i: &Box<GeoS1Interval>) -> bool {
    i.lo > i.hi
}

