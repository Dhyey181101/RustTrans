

use std::f64;

const GEO_S1_EPSILON: f64 = 1e-15;

fn approx_equal(i: &geo_s1_Interval, other: &geo_s1_Interval) -> bool {
    if is_empty(i) {
        return other.length() <= 2.0 * GEO_S1_EPSILON;
    }
    if is_empty(other) {
        return i.length() <= 2.0 * GEO_S1_EPSILON;
    }
    if is_full(i) {
        return other.length() >= 2.0 * (f64::consts::PI - GEO_S1_EPSILON);
    }
    if is_full(other) {
        return i.length() >= 2.0 * (f64::consts::PI - GEO_S1_EPSILON);
    }

    (f64::abs(i.lo.rem_euclid(2.0 * f64::consts::PI)) <= GEO_S1_EPSILON &&
     f64::abs(i.hi.rem_euclid(2.0 * f64::consts::PI)) <= GEO_S1_EPSILON &&
     f64::abs(i.length() - other.length()) <= 2.0 * GEO_S1_EPSILON)
}

fn is_empty(i: &geo_s1_Interval) -> bool {
    i.lo == f64::consts::PI && i.hi == -f64::consts::PI
}

fn length(i: &geo_s1_Interval) -> f64 {
        let l = i.hi - i.lo;
        if l >= 0.0 {
            return l;
        }
        l + 2.0 * f64::consts::PI
    }

fn is_full(i: &geo_s1_Interval) -> bool {
    i.lo == -f64::consts::PI && i.hi == f64::consts::PI
}

#[derive(Copy, Clone)]
struct geo_s1_Interval {
    lo: f64,
    hi: f64,
}

impl geo_s1_Interval {
    fn length(&self) -> f64 {
        let l = self.hi - self.lo;
        if l >= 0.0 {
            return l;
        }
        l + 2.0 * f64::consts::PI
    }
}

