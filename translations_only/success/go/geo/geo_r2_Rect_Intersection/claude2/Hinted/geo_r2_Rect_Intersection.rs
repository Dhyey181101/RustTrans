
use std::f64::{self, MAX, MIN}; 

struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

struct GeoR2Rect {
    x: GeoR1Interval,
    y: GeoR1Interval, 
}

fn intersection(r: &GeoR2Rect, other: &GeoR2Rect) -> GeoR2Rect {
    let xx = intersect_intervals(&r.x, &other.x);
    let yy = intersect_intervals(&r.y, &other.y);
    if is_empty(&xx) || is_empty(&yy) {
        empty_rect()
    } else {
        GeoR2Rect { x: xx, y: yy }
    }
}

fn intersect_intervals(i: &GeoR1Interval, j: &GeoR1Interval) -> GeoR1Interval {
    GeoR1Interval {
        lo: f64::max(i.lo, j.lo),
        hi: f64::min(i.hi, j.hi),
    }
}

fn is_empty(i: &GeoR1Interval) -> bool {
    i.lo > i.hi
}

fn empty_rect() -> GeoR2Rect {
    GeoR2Rect {
        x: empty_interval(),
        y: empty_interval(), 
    }
}

fn empty_interval() -> GeoR1Interval {
    GeoR1Interval { lo: 1.0, hi: 0.0 }  
}

