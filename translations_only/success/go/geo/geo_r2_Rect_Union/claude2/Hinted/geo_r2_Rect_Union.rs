
use std::cmp::{self, Ordering};

#[derive(Clone)]
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

fn geo_r1_interval_union(i: &GeoR1Interval, other: &GeoR1Interval) -> GeoR1Interval {
    if geo_r1_interval_is_empty(i) {
        other.clone()
    } else if geo_r1_interval_is_empty(other) {
        i.clone()
    } else {
        GeoR1Interval {
            lo: min(i.lo, other.lo),
            hi: max(i.hi, other.hi),
        }
    }
}

fn min(x: f64, y: f64) -> f64 {
    match x.partial_cmp(&y) {
        Some(Ordering::Less) => x,
        _ => y,
    }
}

fn max(x: f64, y: f64) -> f64 {
    match x.partial_cmp(&y) {
        Some(Ordering::Greater) => x,
        _ => y,
    }
}

fn geo_r2_rect_union(r: &GeoR2Rect, other: &GeoR2Rect) -> GeoR2Rect {
    GeoR2Rect {
        x: geo_r1_interval_union(&r.x, &other.x),
        y: geo_r1_interval_union(&r.y, &other.y),
    }
}

