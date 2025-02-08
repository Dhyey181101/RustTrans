
use std::f64::consts::PI;

pub fn contains(i: geo_s1_Interval, p: f64) -> bool {
    let mut p = p;
    if p == -PI {
        p = PI;
    }
    fast_contains(i, p)
}

pub fn fast_contains(i: geo_s1_Interval, p: f64) -> bool {
    if is_inverted(&i) {
        (p >= i.lo || p <= i.hi) && !is_empty(&i)
    } else {
        p >= i.lo && p <= i.hi
    }
}

pub fn is_inverted(i: &geo_s1_Interval) -> bool {
    i.lo > i.hi
}

pub fn is_empty(i: &geo_s1_Interval) -> bool {
    i.lo == PI && i.hi == -PI
}

#[derive(Debug)]
pub struct geo_s1_Interval {
    pub lo: f64,
    pub hi: f64,
}
