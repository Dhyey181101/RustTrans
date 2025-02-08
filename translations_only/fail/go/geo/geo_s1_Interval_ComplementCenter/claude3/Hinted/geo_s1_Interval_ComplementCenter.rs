

use std::f64::consts::PI;

#[derive(Clone, Copy)]
struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

fn geo_s1_full_interval() -> GeoS1Interval {
    GeoS1Interval {
        lo: -PI,
        hi: PI,
    }
}

fn complement_center(i: GeoS1Interval) -> Option<f64> {
    if i.lo != i.hi {
        return Some(center(complement(i)));
    }
    // Singleton. The interval just contains a single point.
    if i.hi <= 0.0 {
        return Some(i.hi + PI);
    }
    Some(i.hi - PI)
}

fn complement(i: GeoS1Interval) -> GeoS1Interval {
    if i.lo == i.hi {
        // Singleton. The interval just contains a single point.
        return geo_s1_full_interval();
    }
    // Handles empty and full.
    GeoS1Interval { lo: i.hi, hi: i.lo }
}

fn center(i: GeoS1Interval) -> f64 {
    let c = 0.5 * (i.lo + i.hi);
    if !is_inverted(i) {
        return c;
    }
    if c <= 0.0 {
        return c + PI;
    }
    c - PI
}

fn is_inverted(i: GeoS1Interval) -> bool {
    i.lo > i.hi
}

