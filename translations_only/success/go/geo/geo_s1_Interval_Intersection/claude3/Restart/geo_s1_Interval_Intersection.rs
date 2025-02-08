

use std::f64::consts::PI;

fn intersection(i: Interval, oi: Interval) -> Interval {
    if is_empty(&oi) {
        return empty_interval();
    }
    if fast_contains(&i, oi.lo) {
        if fast_contains(&i, oi.hi) {
            if length(&oi) < length(&i) {
                return oi;
            }
            return i;
        }
        return Interval { lo: oi.lo, hi: i.hi };
    }
    if fast_contains(&i, oi.hi) {
        return Interval { lo: i.lo, hi: oi.hi };
    }

    if fast_contains(&oi, i.lo) {
        return i;
    }
    return empty_interval();
}

fn is_empty(i: &Interval) -> bool {
    i.lo == PI && i.hi == -PI
}

fn empty_interval() -> Interval {
    Interval { lo: PI, hi: -PI }
}

fn fast_contains(i: &Interval, p: f64) -> bool {
    if is_inverted(i) {
        return (p >= i.lo || p <= i.hi) && !is_empty(i);
    }
    p >= i.lo && p <= i.hi
}

fn is_inverted(i: &Interval) -> bool {
    i.lo > i.hi
}

fn length(i: &Interval) -> f64 {
    let mut l = i.hi - i.lo;
    if l >= 0.0 {
        return l;
    }
    l += 2.0 * PI;
    if l > 0.0 {
        return l;
    }
    -1.0
}

#[derive(Copy, Clone)]
struct Interval {
    lo: f64,
    hi: f64,
}

