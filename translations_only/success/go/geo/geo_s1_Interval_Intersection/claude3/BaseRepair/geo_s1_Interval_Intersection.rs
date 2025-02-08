
use std::f64::consts::PI;

fn geo_s1_intersection(i: geo_s1_Interval, oi: geo_s1_Interval) -> geo_s1_Interval {
    if geo_s1_is_empty(&oi) {
        return geo_s1_empty_interval();
    }
    if geo_s1_fast_contains(&i, oi.lo) {
        if geo_s1_fast_contains(&i, oi.hi) {
            if geo_s1_length(&oi) < geo_s1_length(&i) {
                return oi;
            }
            return i;
        }
        return geo_s1_Interval { lo: oi.lo, hi: i.hi };
    }
    if geo_s1_fast_contains(&i, oi.hi) {
        return geo_s1_Interval { lo: i.lo, hi: oi.hi };
    }

    if geo_s1_fast_contains(&oi, i.lo) {
        return i;
    }
    geo_s1_empty_interval()
}

fn geo_s1_is_empty(i: &geo_s1_Interval) -> bool {
    i.lo == PI && i.hi == -PI
}

fn geo_s1_empty_interval() -> geo_s1_Interval {
    geo_s1_Interval { lo: PI, hi: -PI }
}

fn geo_s1_fast_contains(i: &geo_s1_Interval, p: f64) -> bool {
    if geo_s1_is_inverted(i) {
        return (p >= i.lo || p <= i.hi) && !geo_s1_is_empty(i);
    }
    p >= i.lo && p <= i.hi
}

fn geo_s1_is_inverted(i: &geo_s1_Interval) -> bool {
    i.lo > i.hi
}

fn geo_s1_length(i: &geo_s1_Interval) -> f64 {
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
