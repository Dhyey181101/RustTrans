

use std::f64::consts::PI;

const GEO_S1_EPSILON: f64 = 1e-15;

fn geo_s1_interval_approx_equal(i: GeoS1Interval, other: GeoS1Interval) -> bool {
    if geo_s1_interval_is_empty(i) {
        return geo_s1_interval_length(other) <= 2.0 * GEO_S1_EPSILON;
    }
    if geo_s1_interval_is_empty(other) {
        return geo_s1_interval_length(i) <= 2.0 * GEO_S1_EPSILON;
    }
    if geo_s1_interval_is_full(i) {
        return geo_s1_interval_length(other) >= 2.0 * (PI - GEO_S1_EPSILON);
    }
    if geo_s1_interval_is_full(other) {
        return geo_s1_interval_length(i) >= 2.0 * (PI - GEO_S1_EPSILON);
    }

    (
        (other.lo - i.lo).rem_euclid(2.0 * PI).abs() <= GEO_S1_EPSILON
            && (other.hi - i.hi).rem_euclid(2.0 * PI).abs() <= GEO_S1_EPSILON
            && (geo_s1_interval_length(i) - geo_s1_interval_length(other)).abs() <= 2.0 * GEO_S1_EPSILON
    )
}

fn geo_s1_interval_is_empty(i: GeoS1Interval) -> bool {
    i.lo == PI && i.hi == -PI
}

fn geo_s1_interval_length(i: GeoS1Interval) -> f64 {
    let mut l = i.hi - i.lo;
    if l >= 0.0 {
        return l;
    }
    l += 2.0 * PI;
    if l > 0.0 {
        return l;
    }
    return -1.0;
}

fn geo_s1_interval_is_full(i: GeoS1Interval) -> bool {
    i.lo == -PI && i.hi == PI
}

#[derive(Copy, Clone)]
struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

