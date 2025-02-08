
use std::f64::EPSILON;

const GEO_S2_EPSILON: f64 = 1e-15;
const GEO_R1_EPSILON: f64 = 1e-15;

struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

fn geo_r1_interval_approx_equal(i: Box<GeoR1Interval>, other: Box<GeoR1Interval>) -> bool {
    if geo_r1_interval_is_empty(i.as_ref()) {
        return geo_r1_interval_length(other.as_ref()) <= 2.0 * GEO_R1_EPSILON;
    }
    if geo_r1_interval_is_empty(other.as_ref()) {
        return geo_r1_interval_length(i.as_ref()) <= 2.0 * GEO_R1_EPSILON;
    }
    return (other.lo - i.lo).abs() <= GEO_R1_EPSILON
        && (other.hi - i.hi).abs() <= GEO_R1_EPSILON;
}

fn geo_r1_interval_is_empty(i: &GeoR1Interval) -> bool {
    i.lo > i.hi
}

fn geo_r1_interval_length(i: &GeoR1Interval) -> f64 {
    i.hi - i.lo
}
