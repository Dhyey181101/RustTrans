

use std::f64::consts::PI;

struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

fn add_point(interval: Box<GeoS1Interval>, mut p: f64) -> Box<GeoS1Interval> {
    if p.abs() > PI {
        return interval;
    }
    if p == -PI {
        p = PI;
    }
    if fast_contains(&interval, p) {
        return interval;
    }
    if is_empty(&interval) {
        return Box::new(GeoS1Interval {
            lo: p,
            hi: p,
        });
    }
    if positive_distance(p, interval.lo) < positive_distance(interval.hi, p) {
        return Box::new(GeoS1Interval {
            lo: p,
            hi: interval.hi,
        });
    }
    return Box::new(GeoS1Interval {
        lo: interval.lo,
        hi: p,
    });
}

fn fast_contains(interval: &GeoS1Interval, p: f64) -> bool {
    if is_inverted(interval) {
        return (p >= interval.lo || p <= interval.hi) && !is_empty(interval);
    }
    p >= interval.lo && p <= interval.hi
}

fn is_inverted(interval: &GeoS1Interval) -> bool {
    interval.lo > interval.hi
}

fn is_empty(interval: &GeoS1Interval) -> bool {
    interval.lo == PI && interval.hi == -PI
}

fn positive_distance(a: f64, b: f64) -> f64 {
    let d = b - a;
    if d >= 0.0 {
        return d;
    }
    (b + PI) - (a - PI)
}

