

use std::f64::{self, MAX, MIN};

struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

struct GeoR2Rect {
    x: GeoR1Interval,
    y: GeoR1Interval,
}

fn geo_r1_interval_union(i: GeoR1Interval, other: GeoR1Interval) -> GeoR1Interval {
    if is_geo_r1_interval_empty(&i) {
        return other;
    }
    if is_geo_r1_interval_empty(&other) {
        return i;
    }
    GeoR1Interval {
        lo: f64::min(i.lo, other.lo),
        hi: f64::max(i.hi, other.hi),
    }
}

fn geo_r2_rect_union(r: GeoR2Rect, other: GeoR2Rect) -> GeoR2Rect {
    GeoR2Rect {
        x: geo_r1_interval_union(r.x, other.x),
        y: geo_r1_interval_union(r.y, other.y),
    }
}

fn is_geo_r1_interval_empty(i: &GeoR1Interval) -> bool {
    i.lo > i.hi
}

