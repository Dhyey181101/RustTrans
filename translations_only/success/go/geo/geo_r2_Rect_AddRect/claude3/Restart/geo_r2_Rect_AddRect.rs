

use std::f64;

struct GeoR2Rect {
    x: Box<GeoR1Interval>,
    y: Box<GeoR1Interval>,
}

struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

fn geo_r2_rect_add_rect(r: GeoR2Rect, other: GeoR2Rect) -> GeoR2Rect {
    GeoR2Rect {
        x: Box::new(geo_r1_interval_union(*r.x, *other.x)),
        y: Box::new(geo_r1_interval_union(*r.y, *other.y)),
    }
}

fn geo_r1_interval_union(i: GeoR1Interval, other: GeoR1Interval) -> GeoR1Interval {
    if geo_r1_interval_is_empty(&i) {
        return other;
    }
    if geo_r1_interval_is_empty(&other) {
        return i;
    }
    GeoR1Interval {
        lo: f64::min(i.lo, other.lo),
        hi: f64::max(i.hi, other.hi),
    }
}

fn geo_r1_interval_is_empty(i: &GeoR1Interval) -> bool {
    i.lo > i.hi
}

