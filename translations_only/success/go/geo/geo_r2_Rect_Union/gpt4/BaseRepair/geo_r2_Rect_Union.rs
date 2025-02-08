
use std::f64;

struct GeoR2Rect {
    x: GeoR1Interval,
    y: GeoR1Interval,
}

struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

impl GeoR2Rect {
    fn union(&self, other: &GeoR2Rect) -> GeoR2Rect {
        GeoR2Rect {
            x: union_r1(&self.x, &other.x),
            y: union_r1(&self.y, &other.y),
        }
    }
}

impl GeoR1Interval {
    fn is_empty(&self) -> bool {
        self.lo > self.hi
    }
}

fn union_r1(a: &GeoR1Interval, b: &GeoR1Interval) -> GeoR1Interval {
    if a.is_empty() {
        return GeoR1Interval { lo: b.lo, hi: b.hi };
    }
    if b.is_empty() {
        return GeoR1Interval { lo: a.lo, hi: a.hi };
    }
    GeoR1Interval {
        lo: a.lo.min(b.lo),
        hi: a.hi.max(b.hi),
    }
}
