
use std::f64::consts::PI;

struct Interval {
    lo: f64,
    hi: f64,
}

fn intersection(mut i: Interval, oi: Interval) -> Interval {
    if is_empty(&oi) {
        return empty_interval();
    }
    if contains(&i, oi.lo) {
        if contains(&i, oi.hi) {
            if length(&oi) < length(&i) {
                return oi;
            }
            return i;
        }
        return Interval {
            lo: oi.lo,
            hi: i.hi,
        };
    }
    if contains(&i, oi.hi) {
        return Interval {
            lo: i.lo,
            hi: oi.hi,
        };
    }

    if contains(&oi, i.lo) {
        return i;
    }
    return empty_interval();
}

fn is_empty(i: &Interval) -> bool {
    i.lo == PI && i.hi == -PI  
}

fn empty_interval() -> Interval {
    Interval {
        lo: PI,
        hi: -PI
    }
}

fn contains(i: &Interval, p: f64) -> bool {
    if is_inverted(i) {
        (p >= i.lo || p <= i.hi) && !is_empty(i)
    } else {
        p >= i.lo && p <= i.hi
    }
}

fn is_inverted(i: &Interval) -> bool {
    i.lo > i.hi
}

fn length(i: &Interval) -> f64 {
    let l = i.hi - i.lo;
    if l >= 0.0 {
        return l;
    }
    let l = l + 2.0 * PI;
    if l > 0.0 {
        return l;
    }
    -1.0  
}

