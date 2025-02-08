

use std::cmp::Ordering;
use std::cmp::min;
use std::cmp::max;

#[derive(Copy, Clone, Debug)]
struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

impl GeoR1Interval {
    fn is_empty(&self) -> bool {
        self.lo > self.hi
    }

    fn union(&self, other: GeoR1Interval) -> GeoR1Interval {
        if self.is_empty() {
            return other;
        }
        if other.is_empty() {
            return *self;
        }
        GeoR1Interval {
            lo: f64::min(self.lo, other.lo),
            hi: f64::max(self.hi, other.hi),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct GeoR2Rect {
    x: GeoR1Interval,
    y: GeoR1Interval,
}

fn add_rect(r1: &GeoR2Rect, r2: &GeoR2Rect) -> GeoR2Rect {
    GeoR2Rect {
        x: r1.x.union(r2.x),
        y: r1.y.union(r2.y),
    }
}

