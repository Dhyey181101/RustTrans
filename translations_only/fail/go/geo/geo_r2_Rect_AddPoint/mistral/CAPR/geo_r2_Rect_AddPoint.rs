

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

fn add_point_to_geor1interval(geor1interval: &mut GeoR1Interval, p: f64) {
    if geor1interval.is_empty() {
        geor1interval.lo = p;
        geor1interval.hi = p;
    } else if p < geor1interval.lo {
        geor1interval.lo = p;
    } else if p > geor1interval.hi {
        geor1interval.hi = p;
    }
}

fn add_point_to_geor2rect(geor2rect: &mut GeoR2Rect, p: GeoR2Point) {
    add_point_to_geor1interval(&mut geor2rect.x, p.x);
    add_point_to_geor1interval(&mut geor2rect.y, p.y);
}

impl GeoR1Interval {
    fn is_empty(&self) -> bool {
        self.lo > self.hi
    }
}

