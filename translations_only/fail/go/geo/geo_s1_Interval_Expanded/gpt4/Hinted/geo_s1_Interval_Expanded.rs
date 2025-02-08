
use std::f64::consts::PI;

const GEO_S1_DBLEPSILON: f64 = 2.220446049e-16;

struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

fn expanded(interval: &GeoS1Interval, margin: f64) -> GeoS1Interval {
    if margin >= 0.0 {
        if is_empty(interval) {
            return GeoS1Interval { lo: interval.lo, hi: interval.hi };
        }
        if length(interval) + 2.0 * margin + 2.0 * GEO_S1_DBLEPSILON >= 2.0 * PI {
            return full_interval();
        }
    } else {
        if is_full(interval) {
            return GeoS1Interval { lo: interval.lo, hi: interval.hi };
        }
        if length(interval) + 2.0 * margin - 2.0 * GEO_S1_DBLEPSILON <= 0.0 {
            return empty_interval();
        }
    }
    let mut result = interval_from_endpoints(
        ((interval.lo - margin) % (2.0 * PI) + (2.0 * PI)) % (2.0 * PI),
        ((interval.hi + margin) % (2.0 * PI) + (2.0 * PI)) % (2.0 * PI),
    );
    if result.lo <= -PI {
        result.lo = PI;
    }
    result
}

fn is_empty(interval: &GeoS1Interval) -> bool {
    interval.lo == PI && interval.hi == -PI
}

fn interval_from_endpoints(lo: f64, hi: f64) -> GeoS1Interval {
    let mut interval = GeoS1Interval { lo, hi };
    if lo == -PI && hi != PI {
        interval.lo = PI;
    }
    if hi == -PI && lo != PI {
        interval.hi = PI;
    }
    interval
}

fn is_full(interval: &GeoS1Interval) -> bool {
    interval.lo == -PI && interval.hi == PI
}

fn length(interval: &GeoS1Interval) -> f64 {
    let mut l = interval.hi - interval.lo;
    if l >= 0.0 {
        return l;
    }
    l += 2.0 * PI;
    if l > 0.0 {
        return l;
    }
    -1.0
}

fn full_interval() -> GeoS1Interval {
    GeoS1Interval { lo: -PI, hi: PI }
}

fn empty_interval() -> GeoS1Interval {
    GeoS1Interval { lo: PI, hi: -PI }
}
