
use std::f64::consts::PI;

struct Interval {
    lo: f64,
    hi: f64,
}

fn complement(interval: Interval) -> Interval {
    if interval.lo == interval.hi {
        return full_interval();
    }
    Interval {
        hi: interval.lo,
        lo: interval.hi,
    }
}

fn full_interval() -> Interval {
    Interval {
        lo: -PI,
        hi: PI,
    }
}

fn center(interval: Interval) -> f64 {
    let c = 0.5 * (interval.lo + interval.hi);
    if !is_inverted(interval) {
        return c;
    }
    if c <= 0.0 {
        c + PI
    } else {
        c - PI
    }
}

fn is_inverted(interval: Interval) -> bool {
    interval.lo > interval.hi
}

fn complement_center(interval: Interval) -> f64 {
    if interval.lo != interval.hi {
        return center(complement(interval));
    }
    if interval.hi <= 0.0 {
        interval.hi + PI
    } else {
        interval.hi - PI
    }
}
