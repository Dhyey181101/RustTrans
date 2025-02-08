
use std::f64::{self, INFINITY};

#[derive(Clone, Copy)]
struct GeoR2Point {
    x: f64,
    y: f64, 
}

#[derive(Clone, Copy)]
struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

#[derive(Clone, Copy)]
struct GeoR2Rect {
    x: GeoR1Interval,
    y: GeoR1Interval,
}

fn expanded(r: GeoR2Rect, margin: GeoR2Point) -> GeoR2Rect {
    let x = expanded_interval(r.x, margin.x);
    let y = expanded_interval(r.y, margin.y);
    if is_empty(x) || is_empty(y) {
        empty_rect()
    } else {
        GeoR2Rect { x, y }
    }
}

fn expanded_interval(i: GeoR1Interval, margin: f64) -> GeoR1Interval {
    if is_empty(i) {
        i
    } else {
        GeoR1Interval {
            lo: i.lo - margin,
            hi: i.hi + margin,
        }
    }
}

fn is_empty(i: GeoR1Interval) -> bool {
    i.lo > i.hi
}

fn empty_rect() -> GeoR2Rect {
    GeoR2Rect {
        x: empty_interval(),
        y: empty_interval(),
    }
}

fn empty_interval() -> GeoR1Interval {
    GeoR1Interval {
        lo: INFINITY,
        hi: f64::NEG_INFINITY,
    }
}

