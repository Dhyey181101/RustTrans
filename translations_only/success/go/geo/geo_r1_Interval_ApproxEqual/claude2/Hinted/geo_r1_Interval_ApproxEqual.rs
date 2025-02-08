
use std::f64::consts::{E, PI};

const GEO_S2_EPSILON: f64 = 1e-15;
const GEO_R1_EPSILON: f64 = 1e-15;

struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

fn approx_equal(i: &GeoR1Interval, other: &GeoR1Interval) -> bool {
    if is_empty(i) {
        return length(other) <= 2.0 * GEO_R1_EPSILON;
    }
    if is_empty(other) {
        return length(i) <= 2.0 * GEO_R1_EPSILON;
    }
    (other.lo - i.lo).abs() <= GEO_R1_EPSILON
        && (other.hi - i.hi).abs() <= GEO_R1_EPSILON
}

fn is_empty(i: &GeoR1Interval) -> bool {
    i.lo > i.hi
}

fn length(i: &GeoR1Interval) -> f64 {
    i.hi - i.lo
}

