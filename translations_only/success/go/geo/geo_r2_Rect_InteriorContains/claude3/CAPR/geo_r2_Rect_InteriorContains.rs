
use std::boxed::Box;

struct GeoR2Rect {
    x: Box<GeoR1Interval>,
    y: Box<GeoR1Interval>,
}

struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

fn geo_r2_rect_interior_contains(r: &GeoR2Rect, other: &GeoR2Rect) -> bool {
    geo_r1_interval_interior_contains_interval(&r.x, &other.x)
        && geo_r1_interval_interior_contains_interval(&r.y, &other.y)
}

fn geo_r1_interval_interior_contains_interval(i: &GeoR1Interval, oi: &GeoR1Interval) -> bool {
    if geo_r1_interval_is_empty(oi) {
        return true;
    }
    i.lo < oi.lo && oi.hi < i.hi
}

fn geo_r1_interval_is_empty(i: &GeoR1Interval) -> bool {
    i.lo > i.hi
}
