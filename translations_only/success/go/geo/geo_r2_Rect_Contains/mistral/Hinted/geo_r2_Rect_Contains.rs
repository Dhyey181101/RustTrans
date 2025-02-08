
use std::ops::*;

#[derive(Copy, Clone, Debug)]
struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

impl GeoR1Interval {
    fn contains_interval(&self, oi: GeoR1Interval) -> bool {
        if oi.is_empty() {
            return true;
        }
        self.lo <= oi.lo && oi.hi <= self.hi
    }

    fn is_empty(&self) -> bool {
        self.lo > self.hi
    }
}

#[derive(Copy, Clone, Debug)]
struct GeoR2Rect {
    x: GeoR1Interval,
    y: GeoR1Interval,
}

fn contains(r1: &GeoR2Rect, r2: &GeoR2Rect) -> bool {
    r1.x.contains_interval(r2.x) && r1.y.contains_interval(r2.y)
}
