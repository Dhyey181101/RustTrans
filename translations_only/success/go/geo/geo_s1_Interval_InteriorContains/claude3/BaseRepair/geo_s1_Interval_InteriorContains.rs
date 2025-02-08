
use std::f64::consts::PI;

fn geo_s1_interval_interior_contains(i: &geo_s1_Interval, p: f64) -> bool {
    let mut p = p;
    if p == -PI {
        p = PI;
    }
    if geo_s1_interval_is_inverted(i) {
        return p > i.lo || p < i.hi;
    }
    (p > i.lo && p < i.hi) || geo_s1_interval_is_full(i)
}

fn geo_s1_interval_is_inverted(i: &geo_s1_Interval) -> bool {
    i.lo > i.hi
}

fn geo_s1_interval_is_full(i: &geo_s1_Interval) -> bool {
    i.lo == -PI && i.hi == PI
}

struct geo_s1_Interval {
    lo: f64,
    hi: f64,
}
