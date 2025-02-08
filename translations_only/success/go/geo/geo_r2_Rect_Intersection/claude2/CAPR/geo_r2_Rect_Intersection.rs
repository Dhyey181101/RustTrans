
use std::f64::{self, MAX, MIN};

struct GeoR1Interval {
    lo: f64,
    hi: f64, 
}

struct GeoR2Rect {
    x: GeoR1Interval,
    y: GeoR1Interval,
}

fn is_empty(interval: &GeoR1Interval) -> bool {
    interval.lo > interval.hi
}

fn empty_interval() -> GeoR1Interval {
    GeoR1Interval { lo: 1.0, hi: 0.0 }  
}

fn empty_rect() -> GeoR2Rect {
    GeoR2Rect {
        x: empty_interval(),
        y: empty_interval(),
    }
}

fn min(a: f64, b: f64) -> f64 {
    if a < b {a} else {b}
}

fn max(a: f64, b: f64) -> f64 {
    if a > b {a} else {b}
}

fn intersection(interval1: &GeoR1Interval, interval2: &GeoR1Interval) -> GeoR1Interval {
    GeoR1Interval {
        lo: max(interval1.lo, interval2.lo),
        hi: min(interval1.hi, interval2.hi),
    }
}

fn rect_intersection(rect1: &GeoR2Rect, rect2: &GeoR2Rect) -> GeoR2Rect {
    let x = intersection(&rect1.x, &rect2.x);
    let y = intersection(&rect1.y, &rect2.y);
    if is_empty(&x) || is_empty(&y) {
        empty_rect()
    } else {
        GeoR2Rect { x, y }
    }
}

