

use std::f64::consts::PI;

fn contains(i: Box<S1Interval>, p: f64) -> bool {
    if p == -PI {
        return fast_contains(i, PI);
    }
    fast_contains(i, p)
}

fn fast_contains(i: Box<S1Interval>, p: f64) -> bool {
    if is_inverted(&*i) {
        return (p >= i.lo || p <= i.hi) && !is_empty(&*i);
    }
    p >= i.lo && p <= i.hi
}

fn is_inverted(i: &S1Interval) -> bool {
    i.lo > i.hi
}

fn is_empty(i: &S1Interval) -> bool {
    i.lo == PI && i.hi == -PI
}

struct S1Interval {
    lo: f64,
    hi: f64,
}

