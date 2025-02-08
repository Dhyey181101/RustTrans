
use std::fmt;

#[derive(Debug)]
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

fn new_rect(x: GeoR1Interval, y: GeoR1Interval) -> GeoR2Rect {
    GeoR2Rect {
        x: Box::new(x),
        y: Box::new(y),
    }
}

fn stringify(r: &GeoR2Rect) -> String {
    format!("[Lo{:?}, Hi{:?}]", lo(r), hi(r))
}

fn lo(r: &GeoR2Rect) -> GeoR2Point {
    GeoR2Point {
        x: r.x.lo,
        y: r.y.lo,
    }
}

fn hi(r: &GeoR2Rect) -> GeoR2Point {
    GeoR2Point {
        x: r.x.hi,
        y: r.y.hi,
    }
}

