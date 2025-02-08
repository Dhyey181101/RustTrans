

use std::ops::Add;

struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

struct GeoR2Rect {
    x: GeoR1Interval,
    y: GeoR1Interval,
}

struct GeoR2Point {
    x: f64,
    y: f64,
}

impl Add for GeoR1Interval {
    type Output = GeoR1Interval;

    fn add(self, other: GeoR1Interval) -> GeoR1Interval {
        GeoR1Interval {
            lo: self.lo + other.lo,
            hi: self.hi + other.hi,
        }
    }
}

impl GeoR1Interval {
    fn is_empty(&self) -> bool {
        self.lo > self.hi
    }

    fn add_point(&self, p: f64) -> GeoR1Interval {
        if self.is_empty() {
            GeoR1Interval { lo: p, hi: p }
        } else if p < self.lo {
            GeoR1Interval { lo: p, hi: self.hi }
        } else if p > self.hi {
            GeoR1Interval { lo: self.lo, hi: p }
        } else {
            GeoR1Interval { lo: self.lo, hi: self.hi }
        }
    }
}

impl GeoR2Rect {
    fn add_point(&self, p: GeoR2Point) -> GeoR2Rect {
        GeoR2Rect {
            x: self.x.add_point(p.x),
            y: self.y.add_point(p.y),
        }
    }
}

