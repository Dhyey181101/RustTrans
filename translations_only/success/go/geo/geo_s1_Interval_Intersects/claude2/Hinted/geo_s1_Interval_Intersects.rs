
use std::f64::consts::PI;

struct Interval {
    lo: f64,
    hi: f64,
}

fn intersects(i: &Interval, oi: &Interval) -> bool {
    if is_empty(i) || is_empty(oi) {
        return false;
    }
    if is_inverted(i) {
        return is_inverted(oi) || oi.lo <= i.hi || oi.hi >= i.lo;
    }
    if is_inverted(oi) {
        return oi.lo <= i.hi || oi.hi >= i.lo;
    }
    oi.lo <= i.hi && oi.hi >= i.lo
}

fn is_empty(i: &Interval) -> bool {
    i.lo == PI && i.hi == -PI
} 

fn is_inverted(i: &Interval) -> bool {
    i.lo > i.hi
}

