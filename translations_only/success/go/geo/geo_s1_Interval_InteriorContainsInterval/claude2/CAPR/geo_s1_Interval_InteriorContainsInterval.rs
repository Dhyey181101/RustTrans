
use std::f64::consts::PI;

struct Interval {
    lo: f64,
    hi: f64,
}

fn is_inverted(i: &Interval) -> bool {
    i.lo > i.hi
}

fn is_empty(i: &Interval) -> bool {
    i.lo == PI && i.hi == -PI
}

fn is_full(i: &Interval) -> bool {
    i.lo == -PI && i.hi == PI 
}

fn interior_contains_interval(i: &Interval, oi: &Interval) -> bool {
    if is_inverted(i) {
        if is_inverted(oi) {
            (oi.lo > i.lo && oi.hi < i.hi) || is_empty(oi)
        } else {
            oi.lo > i.lo || oi.hi < i.hi
        }
    } else if is_inverted(oi) {
        is_full(i) || is_empty(oi)
    } else {
        (oi.lo > i.lo && oi.hi < i.hi) || is_full(i)
    }
}

