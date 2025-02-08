

use std::f64;

struct GeoR2Rect {
    x: Box<GeoR1Interval>,
    y: Box<GeoR1Interval>,
}

struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

fn geo_r2_rect_intersection(r: GeoR2Rect, other: GeoR2Rect) -> GeoR2Rect {
    let xx = geo_r1_interval_intersection(*r.x, *other.x);
    let yy = geo_r1_interval_intersection(*r.y, *other.y);
    if geo_r1_interval_is_empty(&xx) || geo_r1_interval_is_empty(&yy) {
        return geo_r2_empty_rect();
    }

    GeoR2Rect { x: Box::new(xx), y: Box::new(yy) }
}

fn geo_r1_interval_intersection(i: GeoR1Interval, j: GeoR1Interval) -> GeoR1Interval {
    GeoR1Interval {
        lo: f64::max(i.lo, j.lo),
        hi: f64::min(i.hi, j.hi),
    }
}

fn geo_r1_interval_is_empty(i: &GeoR1Interval) -> bool {
    i.lo > i.hi
}

fn geo_r2_empty_rect() -> GeoR2Rect {
    GeoR2Rect {
        x: Box::new(geo_r1_empty_interval()),
        y: Box::new(geo_r1_empty_interval()),
    }
}

fn geo_r1_empty_interval() -> GeoR1Interval {
    GeoR1Interval { lo: 1.0, hi: 0.0 }
}

