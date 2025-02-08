
const GEO_S2_EPSILON: f64 = 1e-15;
const GEO_R1_EPSILON: f64 = 1e-15;

fn geo_r1_interval_approx_equal(i: Box<GeoR1Interval>, other: Box<GeoR1Interval>) -> bool {
    if geo_r1_interval_is_empty(&i) {
        return geo_r1_interval_length(&other) <= 2.0 * GEO_R1_EPSILON;
    }
    if geo_r1_interval_is_empty(&other) {
        return geo_r1_interval_length(&i) <= 2.0 * GEO_R1_EPSILON;
    }
    return (other.hi - i.lo).abs() <= GEO_R1_EPSILON
        && (other.hi - i.hi).abs() <= GEO_R1_EPSILON;
}

fn geo_r1_interval_is_empty(i: &Box<GeoR1Interval>) -> bool {
    i.lo > i.hi
}

fn geo_r1_interval_length(i: &Box<GeoR1Interval>) -> f64 {
    i.hi - i.lo
}

struct GeoR1Interval {
    lo: f64,
    hi: f64,
}
