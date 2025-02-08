
use std::f64::{self, MAX, MIN};

struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

struct GeoR2Rect {
    x: GeoR1Interval,
    y: GeoR1Interval,
}

fn rect_intersection(r: &GeoR2Rect, other: &GeoR2Rect) -> GeoR2Rect {
    let xx = interval_intersection(&r.x, &other.x);
    let yy = interval_intersection(&r.y, &other.y);
    if is_interval_empty(&xx) || is_interval_empty(&yy) {
        empty_rect()
    } else {
        GeoR2Rect { x: xx, y: yy }
    }
}

fn interval_intersection(i: &GeoR1Interval, j: &GeoR1Interval) -> GeoR1Interval {
    GeoR1Interval {
        lo: f64::max(i.lo, j.lo),
        hi: f64::min(i.hi, j.hi),
    }
}

fn is_interval_empty(i: &GeoR1Interval) -> bool {
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

