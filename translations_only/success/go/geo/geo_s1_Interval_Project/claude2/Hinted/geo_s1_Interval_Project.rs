
use std::f64::consts::PI;

struct Interval {
    lo: f64,
    hi: f64,
}

fn project(interval: &Interval, p: f64) -> f64 {
    if p == -PI {
        return PI;
    }
    if fast_contains(interval, p) {
        return p;
    }

    let dlo = positive_distance(p, interval.lo);
    let dhi = positive_distance(interval.hi, p);
    if dlo < dhi {
        interval.lo
    } else {
        interval.hi
    }
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
