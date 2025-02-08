
use std::f64::{self, MIN, MAX};

struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

struct GeoR2Rect {
    x: GeoR1Interval,
    y: GeoR1Interval,
}

fn geo_r1_interval_is_empty(i: &GeoR1Interval) -> bool {
    i.lo > i.hi
}

fn min(a: f64, b: f64) -> f64 {
    if a < b {a} else {b}
}

fn max(a: f64, b: f64) -> f64 {
    if a > b {a} else {b}
}

fn geo_r1_interval_union(mut i: GeoR1Interval, mut other: GeoR1Interval) -> GeoR1Interval {
    if geo_r1_interval_is_empty(&i) {
        return other;
    }
    if geo_r1_interval_is_empty(&other) {
        return i;
    }
    GeoR1Interval {
        lo: min(i.lo, other.lo),
        hi: max(i.hi, other.hi),
    }
}

fn geo_r2_rect_union(mut r: GeoR2Rect, mut other: GeoR2Rect) -> GeoR2Rect {
    GeoR2Rect {
        x: geo_r1_interval_union(r.x, other.x),
        y: geo_r1_interval_union(r.y, other.y),
    }
}

