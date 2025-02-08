
use std::f64::consts::PI;

pub fn project(i: geo_s1_Interval, mut p: f64) -> f64 {
    if p == -PI {
        p = PI;
    }
    if fast_contains(&i, p) {
        return p;
    }
    let dlo = geo_s1_positive_distance(p, i.lo);
    let dhi = geo_s1_positive_distance(i.hi, p);
    if dlo < dhi {
        return i.lo;
    }
    return i.hi;
}

pub fn fast_contains(i: &geo_s1_Interval, p: f64) -> bool {
    if is_inverted(i) {
        return (p >= i.lo || p <= i.hi) && !is_empty(i);
    }
    return p >= i.lo && p <= i.hi;
}

pub fn is_inverted(i: &geo_s1_Interval) -> bool {
    i.lo > i.hi
}

pub fn is_empty(i: &geo_s1_Interval) -> bool {
    i.lo == PI && i.hi == -PI
}

pub fn geo_s1_positive_distance(a: f64, b: f64) -> f64 {
    let d = b - a;
    if d >= 0.0 {
        return d;
    }
    return (b + PI) - (a - PI);
}

#[derive(Debug)]
pub struct geo_s1_Interval {
    pub lo: f64,
    pub hi: f64,
}
