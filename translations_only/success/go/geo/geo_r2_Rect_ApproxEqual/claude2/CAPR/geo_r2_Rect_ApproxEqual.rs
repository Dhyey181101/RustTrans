

use std::f64::consts::{E, PI};

const GEO_S2_EPSILON: f64 = 1e-15;
const GEO_R1_EPSILON: f64 = 1e-15;

struct GeoR2Rect {
    x: Box<GeoR1Interval>,
    y: Box<GeoR1Interval>,
}

struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

fn approx_equal(r1: &GeoR2Rect, r2: &GeoR2Rect) -> bool {
    approx_equal_interval(&r1.x, &r2.x) && approx_equal_interval(&r1.y, &r2.y)
}

fn approx_equal_interval(i1: &GeoR1Interval, i2: &GeoR1Interval) -> bool {
    if is_empty(i1) {
        return length(i2) <= 2.0 * GEO_R1_EPSILON;
    }
    if is_empty(i2) {
        return length(i1) <= 2.0 * GEO_R1_EPSILON;
    }
    (i1.lo - i2.lo).abs() <= GEO_R1_EPSILON && (i1.hi - i2.hi).abs() <= GEO_R1_EPSILON
}

fn is_empty(i: &GeoR1Interval) -> bool {
    i.lo > i.hi
}

fn length(i: &GeoR1Interval) -> f64 {
    i.hi - i.lo
}

