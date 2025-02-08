

use std::f64::consts::PI;

struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

fn add_point(mut i: Box<GeoS1Interval>, mut p: f64) -> Box<GeoS1Interval> {
    if p.abs() > PI {
        return i;
    }
    if p == -PI {
        p = PI;
    }
    if fast_contains(&i, p) {
        return i;
    }
    if is_empty(&i) {
        return Box::new(GeoS1Interval {
            lo: p,
            hi: p,
        });
    }
    if positive_distance(p, i.lo) < positive_distance(i.hi, p) {
        i.hi = p;
        return i;
    }
    i.lo = p;
    i
}

fn fast_contains(i: &GeoS1Interval, p: f64) -> bool {
    if is_inverted(i) {
        (p >= i.lo || p <= i.hi) && !is_empty(i)
    } else {
        p >= i.lo && p <= i.hi
    }
}

fn is_inverted(i: &GeoS1Interval) -> bool {
    i.lo > i.hi
}

fn is_empty(i: &GeoS1Interval) -> bool {
    i.lo == PI && i.hi == -PI
}

fn positive_distance(a: f64, b: f64) -> f64 {
    let d = b - a;
    if d >= 0.0 {
        d
    } else {
        (b + PI) - (a - PI)
    }
}

