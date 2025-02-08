
use std::f64::consts::PI;

const GEO_S1_DBL_EPSILON: f64 = 2.220446049e-16;

fn geo_s1_interval_expanded(i: geo_s1_Interval, margin: f64) -> geo_s1_Interval {
    if margin >= 0.0 {
        if geo_s1_interval_is_empty(i) {
            return i;
        }
        // Check whether this interval will be full after expansion, allowing
        // for a rounding error when computing each endpoint.
        if geo_s1_interval_length(i) + 2.0 * margin + 2.0 * GEO_S1_DBL_EPSILON >= 2.0 * PI {
            return geo_s1_full_interval();
        }
    } else {
        if geo_s1_interval_is_full(i) {
            return i;
        }
        // Check whether this interval will be empty after expansion, allowing
        // for a rounding error when computing each endpoint.
        if geo_s1_interval_length(i) + 2.0 * margin - 2.0 * GEO_S1_DBL_EPSILON <= 0.0 {
            return geo_s1_empty_interval();
        }
    }
    let result = geo_s1_interval_from_endpoints(
        (i.lo - margin).rem_euclid(2.0 * PI),
        (i.hi + margin).rem_euclid(2.0 * PI),
    );
    if result.lo <= -PI {
        return geo_s1_Interval { lo: PI, hi: result.hi };
    }
    result
}

fn geo_s1_interval_is_empty(i: geo_s1_Interval) -> bool {
    i.lo == PI && i.hi == -PI
}

fn geo_s1_interval_from_endpoints(lo: f64, hi: f64) -> geo_s1_Interval {
    let mut i = geo_s1_Interval { lo, hi };
    if lo == -PI && hi != PI {
        i.lo = PI;
    }
    if hi == -PI && lo != PI {
        i.hi = PI;
    }
    i
}

fn geo_s1_interval_is_full(i: geo_s1_Interval) -> bool {
    i.lo == -PI && i.hi == PI
}

fn geo_s1_interval_length(i: geo_s1_Interval) -> f64 {
    let mut l = i.hi - i.lo;
    if l >= 0.0 {
        return l;
    }
    l += 2.0 * PI;
    if l > 0.0 {
        return l;
    }
    -1.0
}

fn geo_s1_full_interval() -> geo_s1_Interval {
    geo_s1_Interval { lo: -PI, hi: PI }
}

fn geo_s1_empty_interval() -> geo_s1_Interval {
    geo_s1_Interval { lo: PI, hi: -PI }
}

#[derive(Clone, Copy)]
struct geo_s1_Interval {
    lo: f64,
    hi: f64,
}
