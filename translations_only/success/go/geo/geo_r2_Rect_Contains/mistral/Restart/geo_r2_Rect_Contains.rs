
use std::ops::*;

#[derive(Copy, Clone, Debug)]
struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

impl GeoR1Interval {
    fn contains_interval(&self, other: GeoR1Interval) -> bool {
        if other.is_empty() {
            return true;
        }
        self.lo <= other.lo && other.hi <= self.hi
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

fn contains(rect: GeoR2Rect, other: GeoR2Rect) -> bool {
    rect.x.contains_interval(other.x) && rect.y.contains_interval(other.y)
}
