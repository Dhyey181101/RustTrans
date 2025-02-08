
use std::ops::*;

#[derive(Copy, Clone, Debug)]
struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

impl GeoR1Interval {
    fn new(lo: f64, hi: f64) -> Self {
        GeoR1Interval { lo, hi }
    }

    fn is_empty(&self) -> bool {
        self.lo > self.hi
    }

    fn intersection(&self, other: &GeoR1Interval) -> Self {
        GeoR1Interval::new(self.lo.max(other.lo), self.hi.min(other.hi))
    }
}

#[derive(Copy, Clone, Debug)]
struct GeoR2Rect {
    x: GeoR1Interval,
    y: GeoR1Interval,
}

impl GeoR2Rect {
    fn empty() -> Self {
        GeoR2Rect {
            x: GeoR1Interval::new(1.0, 0.0),
            y: GeoR1Interval::new(1.0, 0.0),
        }
    }

    fn intersection(&self, other: &GeoR2Rect) -> Self {
        GeoR2Rect {
            x: self.x.intersection(&other.x),
            y: self.y.intersection(&other.y),
        }
    }
}
