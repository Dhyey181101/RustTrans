
const GEO_S1_EPSILON: f64 = 1e-15;

fn approx_equal(i: &geo_s1_interval, other: &geo_s1_interval) -> bool {
    if is_empty(i) {
        return length(other) <= 2.0 * GEO_S1_EPSILON;
    }
    if is_empty(other) {
        return length(i) <= 2.0 * GEO_S1_EPSILON;
    }
    if is_full(i) {
        return length(other) >= 2.0 * (std::f64::consts::PI - GEO_S1_EPSILON);
    }
    if is_full(other) {
        return length(i) >= 2.0 * (std::f64::consts::PI - GEO_S1_EPSILON);
    }

    let abs_remainder_lo = f64::abs(other.lo - i.lo).rem_euclid(2.0 * std::f64::consts::PI);
    let abs_remainder_hi = f64::abs(other.hi - i.hi).rem_euclid(2.0 * std::f64::consts::PI);
    let abs_length_diff = f64::abs(length(i) - length(other));

    abs_remainder_lo <= GEO_S1_EPSILON
        && abs_remainder_hi <= GEO_S1_EPSILON
        && abs_length_diff <= 2.0 * GEO_S1_EPSILON
}

fn is_empty(i: &geo_s1_interval) -> bool {
    i.lo == std::f64::consts::PI && i.hi == -std::f64::consts::PI
}

fn length(i: &geo_s1_interval) -> f64 {
    let l = i.hi - i.lo;
    if l >= 0.0 {
        return l;
    }
    let l = l + 2.0 * std::f64::consts::PI;
    if l > 0.0 {
        return l;
    }
    -1.0
}

fn is_full(i: &geo_s1_interval) -> bool {
    i.lo == -std::f64::consts::PI && i.hi == std::f64::consts::PI
}

#[derive(Debug, Clone, Copy)]
struct geo_s1_interval {
    lo: f64,
    hi: f64,
}
