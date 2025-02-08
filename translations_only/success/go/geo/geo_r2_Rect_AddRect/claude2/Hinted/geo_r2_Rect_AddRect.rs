
use std::f64::{self, INFINITY, NEG_INFINITY}; 

#[derive(Clone)]
struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

struct GeoR2Rect {
    x: Box<GeoR1Interval>,
    y: Box<GeoR1Interval>,  
}

fn add_rect(r: &GeoR2Rect, other: &GeoR2Rect) -> GeoR2Rect {
    GeoR2Rect {
        x: Box::new(union_interval(&*r.x, &*other.x)),
        y: Box::new(union_interval(&*r.y, &*other.y)),
    }
}

fn union_interval(i: &GeoR1Interval, other: &GeoR1Interval) -> GeoR1Interval {
    if is_empty(i) {
        other.clone()
    } else if is_empty(other) {
        i.clone()
    } else {
        GeoR1Interval {
            lo: f64::min(i.lo, other.lo),
            hi: f64::max(i.hi, other.hi),
        }
    }
}

fn is_empty(i: &GeoR1Interval) -> bool {
    i.lo > i.hi
}

