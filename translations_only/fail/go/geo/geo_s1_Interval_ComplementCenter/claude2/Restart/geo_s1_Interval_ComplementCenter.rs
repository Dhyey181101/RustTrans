

use std::f64::consts::PI;

struct Interval {
    lo: f64,
    hi: f64,
}

fn complement_center(i: Interval) -> f64 {
    if i.lo != i.hi {
        return interval_center(complement(&i));
    }
    // Singleton. The interval just contains a single point.
    if i.hi <= 0.0 {
        return i.hi + PI;
    }
    return i.hi - PI;
}

fn complement(i: &Interval) -> Interval {
    if i.lo == i.hi {
        // Singleton. The interval just contains a single point.
        return full_interval();
    }
    // Handles empty and full.
    Interval {
        hi: i.lo,
        lo: i.hi,
    }
}

fn full_interval() -> Interval {
    Interval {
        lo: -PI,
        hi: PI,
    }
}

fn interval_center(i: Interval) -> f64 {
    let c = 0.5 * (i.lo + i.hi);
    if !is_inverted(i) {
        return c;
    }
    if c <= 0.0 {
        return c + PI;
    }
    return c - PI;
}

fn is_inverted(i: Interval) -> bool {
    i.lo > i.hi
}

