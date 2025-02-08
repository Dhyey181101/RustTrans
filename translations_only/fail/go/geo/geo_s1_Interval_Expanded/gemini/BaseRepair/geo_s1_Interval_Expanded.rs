
use std::f64::consts::PI;

const GEO_S1_DBL_EPSILON: f64 = 2.220446049e-16;

pub fn expanded(i: Interval, margin: f64) -> Interval {
    let mut result = interval_from_endpoints(i.lo - margin, i.hi + margin);
    if margin >= 0.0 {
        if is_empty(&i) {
            return i;
        }
        if length(&i) + 2.0 * margin + 2.0 * GEO_S1_DBL_EPSILON >= 2.0 * PI {
            return full_interval();
        }
    } else {
        if is_full(&i) {
            return i;
        }
        if length(&i) + 2.0 * margin - 2.0 * GEO_S1_DBL_EPSILON <= 0.0 {
            return empty_interval();
        }
    }
    if result.lo <= -PI {
        result.lo = PI;
    }
    result
}

pub fn is_empty(i: &Interval) -> bool {
    i.lo == PI && i.hi == -PI
}

pub fn interval_from_endpoints(lo: f64, hi: f64) -> Interval {
    let mut i = Interval { lo, hi };
    if lo == -PI && hi != PI {
        i.lo = PI;
    }
    if hi == -PI && lo != PI {
        i.hi = PI;
    }
    i
}

pub fn is_full(i: &Interval) -> bool {
    i.lo == -PI && i.hi == PI
}

pub fn length(i: &Interval) -> f64 {
    let l = i.hi - i.lo;
    if l >= 0.0 {
        return l;
    }
    l + 2.0 * PI
}

pub fn full_interval() -> Interval {
    Interval { lo: -PI, hi: PI }
}

pub fn empty_interval() -> Interval {
    Interval { lo: PI, hi: -PI }
}

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub lo: f64,
    pub hi: f64,
}
