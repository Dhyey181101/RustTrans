
use std::f64::consts::PI;

fn intersection(i: Box<geo_s1_Interval>, oi: Box<geo_s1_Interval>) -> Box<geo_s1_Interval> {
    if is_empty(&oi) {
        return Box::new(geo_s1_empty_interval());
    }
    if fast_contains(&i, oi.lo) {
        if fast_contains(&i, oi.hi) {
            if length(&oi) < length(&i) {
                return oi;
            }
            return i;
        }
        return Box::new(geo_s1_Interval { lo: oi.lo, hi: i.hi });
    }
    if fast_contains(&i, oi.hi) {
        return Box::new(geo_s1_Interval { lo: i.lo, hi: oi.hi });
    }

    if fast_contains(&oi, i.lo) {
        return i;
    }
    return Box::new(geo_s1_empty_interval());
}

fn is_empty(i: &Box<geo_s1_Interval>) -> bool {
    i.lo == PI && i.hi == -PI
}

fn geo_s1_empty_interval() -> geo_s1_Interval {
    geo_s1_Interval { lo: PI, hi: -PI }
}

fn fast_contains(i: &Box<geo_s1_Interval>, p: f64) -> bool {
    if is_inverted(i) {
        return (p >= i.lo || p <= i.hi) && !is_empty(i);
    }
    p >= i.lo && p <= i.hi
}

fn is_inverted(i: &Box<geo_s1_Interval>) -> bool {
    i.lo > i.hi
}

fn length(i: &Box<geo_s1_Interval>) -> f64 {
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

struct geo_s1_Interval {
    lo: f64,
    hi: f64,
}
