

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

    fn width(&self) -> f64 {
        self.hi - self.lo
    }

    fn midpoint(&self) -> f64 {
        self.lo + self.width() / 2.0
    }
}

impl Sub for &GeoR1Interval {
    type Output = GeoR1Interval;

    fn sub(self, other: &GeoR1Interval) -> GeoR1Interval {
        GeoR1Interval {
            lo: self.lo - other.hi,
            hi: self.hi - other.lo,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct GeoR2Rect {
    x: GeoR1Interval,
    y: GeoR1Interval,
}

impl GeoR2Rect {
    fn union(r: &GeoR2Rect, other: &GeoR2Rect) -> GeoR2Rect {
        GeoR2Rect {
            x: r.x.union(&other.x),
            y: r.y.union(&other.y),
        }
    }

    fn width(&self) -> f64 {
        self.x.width()
    }

    fn height(&self) -> f64 {
        self.y.width()
    }

    fn area(&self) -> f64 {
        self.width() * self.height()
    }

    fn center(&self) -> (f64, f64) {
        (
            self.x.midpoint(),
            self.y.midpoint(),
        )
    }
}

