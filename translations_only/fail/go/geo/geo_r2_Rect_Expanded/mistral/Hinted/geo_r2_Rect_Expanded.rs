
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
    fn expanded(&self, margin: f64) -> Self {
        let (x, y) = (self.0, self.1);
        let xx = x.expanded(margin);
        let yy = y.expanded(margin);
        if xx.is_empty() || yy.is_empty() {
            GeoR2Interval::default()
        } else {
            GeoR2Interval(xx, yy)
        }
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty() || self.1.is_empty()
    }
}

fn geo_r2_empty_rect() -> GeoR2Interval {
    GeoR2Interval(geo_r1_empty_interval(), geo_r1_empty_interval())
}

fn geo_r1_empty_interval() -> GeoR1Interval {
    GeoR1Interval { lo: 1.0, hi: 0.0 }
}
