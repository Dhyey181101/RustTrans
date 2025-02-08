
use std::ops::*;

#[derive(Copy, Clone, Debug)]
struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

impl GeoR1Interval {
    fn is_empty(&self) -> bool {
        self.lo > self.hi
    }

    fn interior_contains_interval(&self, oi: GeoR1Interval) -> bool {
        if oi.is_empty() {
            return true;
        }
        self.lo < oi.lo && oi.hi < self.hi
    }
}

#[derive(Copy, Clone, Debug)]
struct GeoR2Rect {
    x: GeoR1Interval,
    y: GeoR1Interval,
}

impl GeoR2Rect {
    fn interior_contains(&self, other: GeoR2Rect) -> bool {
        self.x.interior_contains_interval(other.x) && self.y.interior_contains_interval(other.y)
    }
}
