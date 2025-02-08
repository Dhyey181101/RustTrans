
use std::f64::consts::PI;

fn geo_s1_interval_add_point(i: GeoS1Interval, p: f64) -> GeoS1Interval {
    if p.abs() > PI {
        return i;
    }
    if p == -PI {
        return geo_s1_interval_add_point(i, PI);
    }
    if geo_s1_interval_fast_contains(i, p) {
        return i;
    }
    if geo_s1_interval_is_empty(i) {
        return GeoS1Interval { lo: p, hi: p };
    }
    if geo_s1_positive_distance(p, i.lo) < geo_s1_positive_distance(i.hi, p) {
        return GeoS1Interval { lo: p, hi: i.hi };
    }
    GeoS1Interval { lo: i.lo, hi: p }
}

fn geo_s1_interval_fast_contains(i: GeoS1Interval, p: f64) -> bool {
    if geo_s1_interval_is_inverted(i) {
        return (p >= i.lo || p <= i.hi) && !geo_s1_interval_is_empty(i);
    }
    p >= i.lo && p <= i.hi
}

fn geo_s1_interval_is_inverted(i: GeoS1Interval) -> bool {
    i.lo > i.hi
}

fn geo_s1_interval_is_empty(i: GeoS1Interval) -> bool {
    i.lo == PI && i.hi == -PI
}

fn geo_s1_positive_distance(a: f64, b: f64) -> f64 {
    let d = b - a;
    if d >= 0.0 {
        d
    } else {
        (b + PI) - (a - PI)
    }
}

#[derive(Clone, Copy)]
struct GeoS1Interval {
    lo: f64,
    hi: f64,
}
