
const GEO_S1_DBLEPSILON: f64 = 2.220446049e-16;

struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

impl GeoS1Interval {
    fn new(lo: f64, hi: f64) -> Self {
        GeoS1Interval { lo, hi }
    }

    fn is_empty(&self) -> bool {
        self.lo == std::f64::consts::PI && self.hi == -std::f64::consts::PI
    }

    fn is_full(&self) -> bool {
        self.lo == -std::f64::consts::PI && self.hi == std::f64::consts::PI
    }

    fn length(&self) -> f64 {
        let mut l = self.hi - self.lo;
        if l >= 0.0 {
            l
        } else {
            l += 2.0 * std::f64::consts::PI;
            if l > 0.0 { l } else { -1.0 }
        }
    }
}

fn geo_s1_interval_from_endpoints(lo: f64, hi: f64) -> GeoS1Interval {
    let mut interval = GeoS1Interval::new(lo, hi);
    if lo == -std::f64::consts::PI && hi != std::f64::consts::PI {
        interval.lo = std::f64::consts::PI;
    }
    if hi == -std::f64::consts::PI && lo != std::f64::consts::PI {
        interval.hi = std::f64::consts::PI;
    }
    interval
}

fn geo_s1_full_interval() -> GeoS1Interval {
    GeoS1Interval::new(-std::f64::consts::PI, std::f64::consts::PI)
}

fn geo_s1_empty_interval() -> GeoS1Interval {
    GeoS1Interval::new(std::f64::consts::PI, -std::f64::consts::PI)
}

fn expanded(interval: &GeoS1Interval, margin: f64) -> GeoS1Interval {
    if margin >= 0.0 {
        if interval.is_empty() {
            return GeoS1Interval::new(interval.lo, interval.hi);
        }
        if interval.length() + 2.0 * margin + 2.0 * GEO_S1_DBLEPSILON >= 2.0 * std::f64::consts::PI {
            return geo_s1_full_interval();
        }
    } else {
        if interval.is_full() {
            return GeoS1Interval::new(interval.lo, interval.hi);
        }
        if interval.length() + 2.0 * margin - 2.0 * GEO_S1_DBLEPSILON <= 0.0 {
            return geo_s1_empty_interval();
        }
    }
    let result = geo_s1_interval_from_endpoints(
        ((interval.lo - margin) % (2.0 * std::f64::consts::PI) + 2.0 * std::f64::consts::PI) % (2.0 * std::f64::consts::PI),
        ((interval.hi + margin) % (2.0 * std::f64::consts::PI) + 2.0 * std::f64::consts::PI) % (2.0 * std::f64::consts::PI),
    );
    let mut result_lo = result.lo;
    if result_lo <= -std::f64::consts::PI {
        result_lo = std::f64::consts::PI;
    }
    GeoS1Interval::new(result_lo, result.hi)
}
