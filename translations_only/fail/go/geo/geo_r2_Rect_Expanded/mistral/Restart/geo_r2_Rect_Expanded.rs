
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
        GeoR1Interval {
            lo: self.lo - margin,
            hi: self.hi + margin,
        }
    }

    fn is_empty(&self) -> bool {
        self.lo > self.hi
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct GeoR2Interval(GeoR1Interval, GeoR1Interval);

impl GeoR2Interval {
    fn expanded(&self, margin: GeoR2Point) -> Self {
        let xx = self.0.expanded(margin.x);
        let yy = self.1.expanded(margin.y);
        if xx.is_empty() || yy.is_empty() {
            GeoR2Interval::default()
        } else {
            GeoR2Interval(xx, yy)
        }
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty() && self.1.is_empty()
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct GeoR2Point {
    x: f64,
    y: f64,
}

fn geo_r2_empty_rect() -> GeoR2Interval {
    GeoR2Interval(geo_r1_empty_interval(), geo_r1_empty_interval())
}

fn geo_r1_empty_interval() -> GeoR1Interval {
    GeoR1Interval { lo: 1.0, hi: 0.0 }
}
