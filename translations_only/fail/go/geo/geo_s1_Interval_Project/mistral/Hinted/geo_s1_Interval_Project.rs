

use std::f64;

const PI: f64 = std::f64::consts::PI;

struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

fn geo_s1_positive_distance(a: f64, b: f64) -> f64 {
    let d = b - a;
    if d >= 0.0 {
        return d;
    }
    return (b + PI) - (a - PI);
}

fn project(i: &GeoS1Interval, p: &mut f64) {
    if *p == -PI {
        *p = PI;
    }
    if is_inverted(i) {
        if (*p >= i.lo || *p <= i.hi) && !is_empty(i) {
            return;
        }
    } else {
        if *p >= i.lo && *p <= i.hi {
            return;
        }
    }
    let dlo = geo_s1_positive_distance(*p, i.lo);
    let dhi = geo_s1_positive_distance(i.hi, *p);
    if dlo < dhi {
        *p = i.lo;
    } else {
        *p = i.hi;
    }
}

fn is_inverted(i: &GeoS1Interval) -> bool {
    return i.lo > i.hi;
}

fn is_empty(i: &GeoS1Interval) -> bool {
    return i.lo == PI && i.hi == -PI;
}

fn main() {
    let mut interval = GeoS1Interval { lo: 2.530886e-317, hi: -2.877216579734241e+305 };
    let mut p = 3.0805e-320;

    if (p < -1e300 || p > 1e300) || (interval.lo < -1e300 || interval.lo > 1e300) || (interval.hi < -1e300 || interval.hi > 1e300) {
        println!("Input is invalid, crash gracefully");
        return;
    }

    project(&interval, &mut p);
    println!("{}", p);
}

