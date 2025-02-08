

use std::ops::*;

#[derive(Clone)]
pub struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

impl GeoR1Interval {
    pub fn union(&self, other: &GeoR1Interval) -> GeoR1Interval {
        if self.is_empty() {
            return other.clone();
        }
        if other.is_empty() {
            return self.clone();
        }
        GeoR1Interval {
            lo: self.lo.min(other.lo),
            hi: self.hi.max(other.hi),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.lo > self.hi
    }
}

pub struct GeoR2Rect {
    x: GeoR1Interval,
    y: GeoR1Interval,
}

pub fn add(r1: &GeoR2Rect, r2: &GeoR2Rect) -> GeoR2Rect {
    GeoR2Rect {
        x: r1.x.union(&r2.x),
        y: r1.y.union(&r2.y),
    }
}

