
use std::fmt;

fn geo_r2_rect_string(r: &GeoR2Rect) -> String {
    format!("[Lo({}, {}), Hi({}, {})]", r.x.lo, r.y.lo, r.x.hi, r.y.hi)
}

fn geo_r2_rect_lo(r: &GeoR2Rect) -> Box<GeoR2Point> {
    Box::new(GeoR2Point {
        x: r.x.lo,
        y: r.y.lo,
    })
}

fn geo_r2_rect_hi(r: &GeoR2Rect) -> Box<GeoR2Point> {
    Box::new(GeoR2Point {
        x: r.x.hi,
        y: r.y.hi,
    })
}

struct GeoR2Rect {
    x: GeoR1Interval,
    y: GeoR1Interval,
}

struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

struct GeoR2Point {
    x: f64,
    y: f64,
}

impl fmt::Display for GeoR2Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
