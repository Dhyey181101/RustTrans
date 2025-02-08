
use std::ops::Add;

struct GeoR2Point {
    x: f64,
    y: f64,
}

struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

struct GeoR2Rect {
    x: Box<GeoR1Interval>,
    y: Box<GeoR1Interval>, 
}

fn add_point(r: GeoR2Rect, p: GeoR2Point) -> GeoR2Rect {
    GeoR2Rect {
        x: Box::new(add_point_interval(*r.x, p.x)),
        y: Box::new(add_point_interval(*r.y, p.y)),
    }
}

fn add_point_interval(mut i: GeoR1Interval, p: f64) -> GeoR1Interval {
    if i.lo > i.hi {
        return GeoR1Interval { lo: p, hi: p };
    }
    if p < i.lo {
        return GeoR1Interval { lo: p, hi: i.hi };
    }
    if p > i.hi {
        return GeoR1Interval { lo: i.lo, hi: p };
    }
    i
}

fn is_empty(i: &GeoR1Interval) -> bool {
    i.lo > i.hi
}

