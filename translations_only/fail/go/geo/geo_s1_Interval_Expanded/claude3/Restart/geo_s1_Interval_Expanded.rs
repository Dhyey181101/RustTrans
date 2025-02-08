

use std::f64::consts::PI;

const GEO_S1_DBL_EPSILON: f64 = 2.220446049e-16;

fn geo_s1_interval_expanded(i: Box<GeoS1Interval>, margin: f64) -> Box<GeoS1Interval> {
    if margin >= 0.0 {
        if geo_s1_interval_is_empty(&i) {
            return i;
        }
        // Check whether this interval will be full after expansion, allowing
        // for a rounding error when computing each endpoint.
        if geo_s1_interval_length(&i) + 2.0 * margin + 2.0 * GEO_S1_DBL_EPSILON >= 2.0 * PI {
            return Box::new(geo_s1_full_interval());
        }
    } else {
        if geo_s1_interval_is_full(&i) {
            return i;
        }
        // Check whether this interval will be empty after expansion, allowing
        // for a rounding error when computing each endpoint.
        if geo_s1_interval_length(&i) + 2.0 * margin - 2.0 * GEO_S1_DBL_EPSILON <= 0.0 {
            return Box::new(geo_s1_empty_interval());
        }
    }
    let result = geo_s1_interval_from_endpoints(
        (i.lo - margin).rem_euclid(2.0 * PI),
        (i.hi + margin).rem_euclid(2.0 * PI),
    );
    if result.lo <= -PI {
        Box::new(GeoS1Interval {
            lo: PI,
            hi: result.hi,
        })
    } else {
        Box::new(result)
    }
}

fn geo_s1_interval_is_empty(i: &GeoS1Interval) -> bool {
    i.lo == PI && i.hi == -PI
}

fn geo_s1_interval_from_endpoints(lo: f64, hi: f64) -> GeoS1Interval {
    let mut i = GeoS1Interval { lo, hi };
    if lo == -PI && hi != PI {
        i.lo = PI;
    }
    if hi == -PI && lo != PI {
        i.hi = PI;
    }
    i
}

fn geo_s1_interval_is_full(i: &GeoS1Interval) -> bool {
    i.lo == -PI && i.hi == PI
}

fn geo_s1_interval_length(i: &GeoS1Interval) -> f64 {
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

fn geo_s1_full_interval() -> GeoS1Interval {
    GeoS1Interval {
        lo: -PI,
        hi: PI,
    }
}

fn geo_s1_empty_interval() -> GeoS1Interval {
    GeoS1Interval {
        lo: PI,
        hi: -PI,
    }
}

#[derive(Clone, Copy)]
struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

