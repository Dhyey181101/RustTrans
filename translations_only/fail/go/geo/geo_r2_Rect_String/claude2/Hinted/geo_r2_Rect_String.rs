
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

impl fmt::Display for GeoR2Rect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Lo{:?}, Hi{:?}]", lo(self), hi(self))
    }
}

