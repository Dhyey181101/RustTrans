
use std::ops::*;

#[derive(Copy, Clone, Debug)]
struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

impl GeoR1Interval {
    fn is_empty(&self) -> bool {
        self.lo > self.hi
    }

    fn intersection(&self, other: &GeoR1Interval) -> GeoR1Interval {
        GeoR1Interval {
            lo: self.lo.max(other.lo),
            hi: self.hi.min(other.hi),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct GeoR2Rect {
    x: GeoR1Interval,
    y: GeoR1Interval,
}

fn geo_r2_empty_rect() -> GeoR2Rect {
    GeoR2Rect {
        x: geo_r1_empty_interval(),
        y: geo_r1_empty_interval(),
    }
}

fn geo_r1_empty_interval() -> GeoR1Interval {
    GeoR1Interval { lo: 1.0, hi: 0.0 }
}

fn geo_r2_intersection(r: &GeoR2Rect, other: &GeoR2Rect) -> GeoR2Rect {
    let xx = r.x.intersection(&other.x);
    let yy = r.y.intersection(&other.y);
    if xx.is_empty() || yy.is_empty() {
        geo_r2_empty_rect()
    } else {
        GeoR2Rect { x: xx, y: yy }
    }
}
