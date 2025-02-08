
use std::f64;

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
struct GeoR2Rect {
    x: GeoR1Interval,
    y: GeoR1Interval,
}

impl GeoR2Rect {
    fn add_rect(&self, other: GeoR2Rect) -> GeoR2Rect {
        GeoR2Rect {
            x: self.x.union(other.x),
            y: self.y.union(other.y),
        }
    }
}
