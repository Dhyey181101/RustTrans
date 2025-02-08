
use std::f64::consts::PI;

const GEO_S1_EPSILON: f64 = 1e-15;

pub fn approx_equal(i: &geo_s1_Interval, other: &geo_s1_Interval) -> bool {
    if is_empty(i) {
        return length(other) <= 2.0 * GEO_S1_EPSILON;
    }
    if is_empty(other) {
        return length(i) <= 2.0 * GEO_S1_EPSILON;
    }
    if is_full(i) {
        return length(other) >= 2.0 * (PI - GEO_S1_EPSILON);
    }
    if is_full(other) {
        return length(i) >= 2.0 * (PI - GEO_S1_EPSILON);
    }

    let abs_lo = (other.lo - i.lo).abs();
    let abs_hi = (other.hi - i.hi).abs();
    let abs_len = (length(i) - length(other)).abs();

    abs_lo <= GEO_S1_EPSILON && abs_hi <= GEO_S1_EPSILON && abs_len <= 2.0 * GEO_S1_EPSILON
}

pub fn is_empty(i: &geo_s1_Interval) -> bool {
    i.lo == PI && i.hi == -PI
}

pub fn length(i: &geo_s1_Interval) -> f64 {
    let l = i.hi - i.lo;
    if l >= 0.0 {
        return l;
    }
    let l = l + 2.0 * PI;
    if l > 0.0 {
        return l;
    }
    -1.0
}

pub fn is_full(i: &geo_s1_Interval) -> bool {
    i.lo == -PI && i.hi == PI
}

#[derive(Debug, Clone, Copy)]
pub struct geo_s1_Interval {
    pub lo: f64,
    pub hi: f64,
}

