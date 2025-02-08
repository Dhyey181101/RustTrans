
use std::ops::RangeInclusive;

#[derive(Copy, Clone, Debug)]
struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

impl GeoR1Interval {
    fn is_empty(&self) -> bool {
        self.lo > self.hi
    }
}

fn union(a: &GeoR1Interval, b: &GeoR1Interval) -> GeoR1Interval {
    if a.is_empty() {
        return *b;
    }
    if b.is_empty() {
        return *a;
    }
    GeoR1Interval {
        lo: a.lo.min(b.lo),
        hi: a.hi.max(b.hi),
    }
}
