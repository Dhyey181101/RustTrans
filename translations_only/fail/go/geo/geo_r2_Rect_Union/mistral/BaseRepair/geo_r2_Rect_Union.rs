
use std::ops::Sub;

#[derive(Debug, Clone, Copy)]
struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

impl GeoR1Interval {
    fn is_empty(&self) -> bool {
        self.lo > self.hi
    }
}

impl Sub for &GeoR1Interval {
    type Output = GeoR1Interval;

    fn sub(self, other: &GeoR1Interval) -> GeoR1Interval {
        if self.is_empty() {
            return *other;
        }

        GeoR1Interval {
            lo: self.lo.min(other.lo),
            hi: self.hi.max(other.hi),
        }
    }
}
