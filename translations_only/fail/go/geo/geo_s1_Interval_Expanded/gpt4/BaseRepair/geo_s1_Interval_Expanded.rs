
use std::f64::consts::PI;

const GEO_S1_DBLEPSILON: f64 = 2.220446049e-16;

#[derive(Clone, Copy)]
struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

impl GeoS1Interval {
    fn expanded(&self, margin: f64) -> GeoS1Interval {
        if margin >= 0.0 {
            if self.is_empty() {
                return *self;
            }
            if self.length() + 2.0 * margin + 2.0 * GEO_S1_DBLEPSILON >= 2.0 * PI {
                return geo_s1_full_interval();
            }
        } else {
            if self.is_full() {
                return *self;
            }
            if self.length() + 2.0 * margin - 2.0 * GEO_S1_DBLEPSILON <= 0.0 {
                return geo_s1_empty_interval();
            }
        }
        let mut result = geo_s1_interval_from_endpoints(
            ((self.lo - margin) % (2.0 * PI) + 2.0 * PI) % (2.0 * PI),
            ((self.hi + margin) % (2.0 * PI) + 2.0 * PI) % (2.0 * PI),
        );
        if result.lo <= -PI {
            result.lo = PI;
        }
        result
    }

    fn is_empty(&self) -> bool {
        self.lo == PI && self.hi == -PI
    }

    fn is_full(&self) -> bool {
        self.lo == -PI && self.hi == PI
    }

    fn length(&self) -> f64 {
        let mut l = self.hi - self.lo;
        if l >= 0.0 {
            return l;
        }
        l += 2.0 * PI;
        if l > 0.0 {
            return l;
        }
        -1.0
    }
}

fn geo_s1_interval_from_endpoints(lo: f64, hi: f64) -> GeoS1Interval {
    let mut interval = GeoS1Interval { lo, hi };
    if lo == -PI && hi != PI {
        interval.lo = PI;
    }
    if hi == -PI && lo != PI {
        interval.hi = PI;
    }
    interval
}

fn geo_s1_full_interval() -> GeoS1Interval {
    GeoS1Interval { lo: -PI, hi: PI }
}

fn geo_s1_empty_interval() -> GeoS1Interval {
    GeoS1Interval { lo: PI, hi: -PI }
}

