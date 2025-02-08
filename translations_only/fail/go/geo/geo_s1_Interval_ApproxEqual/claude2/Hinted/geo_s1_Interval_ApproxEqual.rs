
use std::f64::consts::PI;

const GEO_S1_EPSILON: f64 = 1e-15;

fn approx_equal(i: &Interval, other: &Interval) -> bool {
    // Full and empty intervals require special cases because the endpoints
    // are considered to be positioned arbitrarily.
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

    // The purpose of the last test below is to verify that moving the endpoints
    // does not invert the interval, e.g. [-1e20, 1e20] vs. [1e20, -1e20].
    (f64::abs(f64::rem_euclid(other.lo - i.lo, 2.0 * PI)) <= GEO_S1_EPSILON)
        && (f64::abs(f64::rem_euclid(other.hi - i.hi, 2.0 * PI)) <= GEO_S1_EPSILON)
        && (f64::abs(length(i) - length(other)) <= 2.0 * GEO_S1_EPSILON)
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

