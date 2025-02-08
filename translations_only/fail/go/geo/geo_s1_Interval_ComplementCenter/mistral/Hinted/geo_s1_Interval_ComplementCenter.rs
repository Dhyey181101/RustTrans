

use std::f64;

struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

fn complement(i: &GeoS1Interval) -> GeoS1Interval {
    if i.lo == i.hi {
        geo_s1_full_interval()
    } else if i.lo < i.hi {
        GeoS1Interval { lo: i.hi, hi: i.lo }
    } else {
        GeoS1Interval { lo: -f64::consts::PI, hi: f64::consts::PI }
    }
}

fn geo_s1_full_interval() -> GeoS1Interval {
    GeoS1Interval { lo: -f64::consts::PI, hi: f64::consts::PI }
}

fn center(i: &GeoS1Interval) -> f64 {
    let c = 0.5 * (i.lo + i.hi);
    if !is_inverted(i) {
        c
    } else if c <= 0.0 {
        c + f64::consts::PI
    } else {
        c - f64::consts::PI
    }
}

fn is_inverted(i: &GeoS1Interval) -> bool {
    i.lo > i.hi
}

fn calculate_center(interval: &GeoS1Interval) -> f64 {
    center(interval)
}

fn complement_center(interval: &GeoS1Interval) -> f64 {
    let complemented = complement(interval);
    center(&complemented)
}

fn main() {
    let i = GeoS1Interval { lo: 7.291122073879517e-304, hi: -5.486124068793689e+303 };
    println!("{}", complement_center(&i));

    let i = GeoS1Interval { lo: -2.6100579242577617e-154, hi: -5.486124865852475e+303 };
    println!("{}", complement_center(&i));

    let i = GeoS1Interval { lo: 3.636e-321, hi: 2.15e-320 };
    println!("{}", calculate_center(&i));

    let i = GeoS1Interval { lo: 2.121995652e-314, hi: 8.810781622794001e-280 };
    println!("{}", calculate_center(&i));
}

