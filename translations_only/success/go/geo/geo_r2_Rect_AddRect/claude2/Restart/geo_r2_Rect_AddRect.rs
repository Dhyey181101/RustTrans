
use std::f64::{self, INFINITY};

#[derive(Clone)]
struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

struct GeoR2Rect {
    x: GeoR1Interval,
    y: GeoR1Interval,
}

fn add_rect(r: Box<GeoR2Rect>, other: Box<GeoR2Rect>) -> Box<GeoR2Rect> {
    Box::new(GeoR2Rect {
        x: union_interval(r.x.clone(), other.x.clone()),
        y: union_interval(r.y.clone(), other.y.clone()),
    })
}

fn union_interval(i: GeoR1Interval, other: GeoR1Interval) -> GeoR1Interval {
    if is_empty(i.clone()) {
        other
    } else if is_empty(other.clone()) {
        i
    } else {
        GeoR1Interval {
            lo: f64::min(i.lo, other.lo),
            hi: f64::max(i.hi, other.hi),
        }
    }
}

fn is_empty(i: GeoR1Interval) -> bool {
    i.lo > i.hi
}

