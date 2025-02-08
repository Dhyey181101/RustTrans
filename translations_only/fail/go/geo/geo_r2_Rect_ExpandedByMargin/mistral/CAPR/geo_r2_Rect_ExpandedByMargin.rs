
use std::ops::Deref;

#[derive(Debug, Default, Clone, Copy)]
struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

impl GeoR1Interval {
    fn expanded(&self, margin: f64) -> Self {
        if self.is_empty() {
            return *self;
        }
        Self {
            lo: self.lo - margin,
            hi: self.hi + margin,
        }
    }

    fn is_empty(&self) -> bool {
        self.lo > self.hi
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct GeoR2Point {
    x: f64,
    y: f64,
}

#[derive(Debug, Default, Clone)]
struct GeoR2Rect {
    x: GeoR1Interval,
    y: GeoR1Interval,
}

impl GeoR2Rect {
    fn expanded(&self, margin: f64) -> Self {
        let margin = GeoR2Point {
            x: margin,
            y: margin,
        };
        self.expanded_by_margin(margin)
    }

    fn expanded_by_margin(&self, margin: GeoR2Point) -> Self {
        let xx = self.x.expanded(margin.x);
        let yy = self.y.expanded(margin.y);
        if xx.is_empty() || yy.is_empty() {
            GeoR2EmptyRect
        } else {
            GeoR2Rect { x: xx, y: yy }
        }
    }
}

const GeoR2EmptyRect: GeoR2Rect = GeoR2Rect {
    x: GeoR1EmptyInterval,
    y: GeoR1EmptyInterval,
};

const GeoR1EmptyInterval: GeoR1Interval = GeoR1Interval { lo: 1.0, hi: 0.0 };
