
use std::boxed::Box;

struct GeoR2Rect {
    x: Box<GeoR1Interval>,
    y: Box<GeoR1Interval>,
}

struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

struct GeoR2Point {
    x: f64,
    y: f64,
}

fn add_point_to_geor1interval(interval: &GeoR1Interval, p: f64) -> GeoR1Interval {
    if interval.is_empty() {
        GeoR1Interval { lo: p, hi: p }
    } else if p < interval.lo {
        GeoR1Interval { lo: p, hi: interval.hi }
    } else if p > interval.hi {
        GeoR1Interval { lo: interval.lo, hi: p }
    } else {
        GeoR1Interval {
            lo: interval.lo,
            hi: interval.hi,
        }
    }
}

impl GeoR2Rect {
    fn add_point(&self, p: GeoR2Point) -> GeoR2Rect {
        GeoR2Rect {
            x: Box::new(add_point_to_geor1interval(&self.x, p.x)),
            y: Box::new(add_point_to_geor1interval(&self.y, p.y)),
        }
    }
}

impl GeoR1Interval {
    fn is_empty(&self) -> bool {
        self.lo > self.hi
    }
}
