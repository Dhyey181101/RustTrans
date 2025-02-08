
use std::f64::consts::PI;

fn interior_contains(i: &geo_s1_interval, mut p: f64) -> bool {
    if p == -PI {
        p = PI;
    }
    if is_inverted(i) {
        p > i.lo || p < i.hi
    } else {
        (p > i.lo && p < i.hi) || is_full(i)
    }
}

fn is_inverted(i: &geo_s1_interval) -> bool {
    i.lo > i.hi
}

fn is_full(i: &geo_s1_interval) -> bool {
    i.lo == -PI && i.hi == PI
}

struct geo_s1_interval {
    lo: f64,
    hi: f64,
}
