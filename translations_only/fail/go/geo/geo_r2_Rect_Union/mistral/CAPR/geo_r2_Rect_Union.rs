

use std::ops::Add;

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
}

#[derive(Debug, Clone, Copy)]
struct GeoR2Rect {
    x: GeoR1Interval,
    y: GeoR1Interval,
}

impl Add for GeoR2Rect {
    type Output = GeoR2Rect;

    fn add(self, other: GeoR2Rect) -> GeoR2Rect {
        GeoR2Rect {
            x: self.x.union(&other.x),
            y: self.y.union(&other.y),
        }
    }
}

impl GeoR2Rect {
    fn area(&self) -> f64 {
        self.x.width() * self.y.width()
    }
}

fn geo_r1_interval_union(a: &GeoR1Interval, b: &GeoR1Interval) -> GeoR1Interval {
    a.union(b)
}

fn geo_r2_rect_union(a: GeoR2Rect, b: GeoR2Rect) -> GeoR2Rect {
    a + b
}

fn geo_r2_rect_area(a: &GeoR2Rect) -> f64 {
    a.area()
}

