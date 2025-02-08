

use std::f64;

fn project(i: &geo_s1_Interval, p: f64) -> f64 {
    if p == -f64::consts::PI {
        p
    } else {
        if fast_contains(i, p) {
            p
        } else {
            let dlo = positive_distance(p, i.lo);
            let dhi = positive_distance(i.hi, p);
            if dlo < dhi {
                i.lo
            } else {
                i.hi
            }
        }
    }
}

fn fast_contains(i: &geo_s1_Interval, p: f64) -> bool {
    if is_inverted(i) {
        p >= i.lo || p <= i.hi && !is_empty(i)
    } else {
        p >= i.lo && p <= i.hi
    }
}

fn is_inverted(i: &geo_s1_Interval) -> bool {
    i.lo > i.hi
}

fn is_empty(i: &geo_s1_Interval) -> bool {
    i.lo == f64::consts::PI && i.hi == -f64::consts::PI
}

fn positive_distance(a: f64, b: f64) -> f64 {
    let d = b - a;
    if d >= 0.0 {
        d
    } else {
        (b + f64::consts::PI) - (a - f64::consts::PI)
    }
}

#[derive(Copy, Clone)]
struct geo_s1_Interval {
    lo: f64,
    hi: f64,
}

