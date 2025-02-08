
use std::f64::consts::PI;

const GEO_S1_DBL_EPSILON: f64 = 2.220446049e-16;

struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

fn geo_s1_interval_expanded(i: GeoS1Interval, margin: f64) -> GeoS1Interval {
    if margin >= 0.0 {
        if i.is_empty() {
            return i;
        }
        if i.length() + 2.0 * margin + 2.0 * GEO_S1_DBL_EPSILON >= 2.0 * PI {
            return geo_s1_full_interval();
        }
    } else {
        if i.is_full() {
            return i;
        }
        if i.length() + 2.0 * margin - 2.0 * GEO_S1_DBL_EPSILON <= 0.0 {
            return geo_s1_empty_interval();
        }
    }

    let mut result = geo_s1_interval_from_endpoints(
        (i.lo - margin).rem_euclid(2.0 * PI),
        (i.hi + margin).rem_euclid(2.0 * PI),
    );

    if result.lo <= -PI {
        result.lo = PI;
    }

    result
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

impl GeoS1Interval {
    fn is_empty(&self) -> bool {
        self.lo == PI && self.hi == -PI
    }

    fn is_full(&self) -> bool {
        self.lo == -PI && self.hi == PI
    }

    fn length(&self) -> f64 {
        let l = self.hi - self.lo;
        if l >= 0.0 {
            l
        } else {
            l + 2.0 * PI
        }.max(0.0)
    }
}
