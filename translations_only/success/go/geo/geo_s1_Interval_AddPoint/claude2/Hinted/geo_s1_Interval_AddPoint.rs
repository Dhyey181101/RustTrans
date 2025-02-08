
use std::f64::consts::PI;

#[derive(Clone)]
struct Interval {
    lo: f64,
    hi: f64,
}

fn add_point(interval: &Interval, mut p: f64) -> Box<Interval> {
    if p.abs() > PI {
        return Box::new(interval.clone());
    }
    if p == -PI {
        p = PI;
    }
    if fast_contains(interval, p) {
        return Box::new(interval.clone());
    }
    if is_empty(interval) {
        return Box::new(Interval { lo: p, hi: p });
    }
    if positive_distance(p, interval.lo) < positive_distance(interval.hi, p) {
        return Box::new(Interval { lo: p, hi: interval.hi });
    }
    Box::new(Interval { lo: interval.lo, hi: p })
}

fn fast_contains(interval: &Interval, p: f64) -> bool {
    if is_inverted(interval) {
        (p >= interval.lo || p <= interval.hi) && !is_empty(interval)
    } else {
        p >= interval.lo && p <= interval.hi
    }
}

fn is_inverted(interval: &Interval) -> bool {
    interval.lo > interval.hi  
}

fn is_empty(interval: &Interval) -> bool {
    interval.lo == PI && interval.hi == -PI
}

fn positive_distance(a: f64, b: f64) -> f64 {
    let d = b - a;
    if d >= 0.0 {
        d
    } else {
        (b + PI) - (a - PI)
    }
}
