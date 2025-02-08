

use std::f64::consts::PI;

const GEO_S1_EPSILON: f64 = 1e-15;

fn approx_equal(i: &Interval, other: &Interval) -> bool {
    if is_empty(i) {
        length(other) <= 2.0 * GEO_S1_EPSILON
    } else if is_empty(other) {
        length(i) <= 2.0 * GEO_S1_EPSILON
    } else if is_full(i) {
        length(other) >= 2.0 * (PI - GEO_S1_EPSILON)
    } else if is_full(other) {
        length(i) >= 2.0 * (PI - GEO_S1_EPSILON)
    } else {
        (i.lo - other.lo).rem_euclid(2.0 * PI) <= GEO_S1_EPSILON
            && (i.hi - other.hi).rem_euclid(2.0 * PI) <= GEO_S1_EPSILON
            && (length(i) - length(other)).abs() <= 2.0 * GEO_S1_EPSILON
    }
}

fn is_empty(i: &Interval) -> bool {
    i.lo == PI && i.hi == -PI 
}

fn length(i: &Interval) -> f64 {
    let l = i.hi - i.lo;
    if l >= 0.0 {
        l
    } else {
        l + 2.0 * PI
    }.max(0.0)
}

fn is_full(i: &Interval) -> bool {
    i.lo == -PI && i.hi == PI
}

struct Interval {
    lo: f64,
    hi: f64
}

