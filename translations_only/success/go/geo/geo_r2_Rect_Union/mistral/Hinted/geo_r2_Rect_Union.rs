

use std::ops::*;

#[derive(Debug, Clone, Copy)]
struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

impl GeoR1Interval {
    fn is_empty(&self) -> bool {
        self.lo > self.hi
    }

    fn union(&self, other: &GeoR1Interval) -> GeoR1Interval {
        if self.is_empty() {
            return *other;
        }
        if other.is_empty() {
            return *self;
        }
        GeoR1Interval {
            lo: self.lo.min(other.lo),
            hi: self.hi.max(other.hi),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct GeoR2Rect {
    x: GeoR1Interval,
    y: GeoR1Interval,
}

fn union(r: GeoR2Rect, other: GeoR2Rect) -> GeoR2Rect {
    GeoR2Rect {
        x: r.x.union(&other.x),
        y: r.y.union(&other.y),
    }
}

