

use std::boxed::Box;

struct GeoR2Rect {
    x: GeoR1Interval,
    y: GeoR1Interval,
}

struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

struct GeoR2Point {
    x: f64,
    y: f64,
}

fn expanded(r: &GeoR2Rect, margin: &GeoR2Point) -> GeoR2Rect {
    let xx = expand_interval(&r.x, &margin.x);
    let yy = expand_interval(&r.y, &margin.y);
    if xx.is_empty() || yy.is_empty() {
        return geo_r2_empty_rect();
    }
    GeoR2Rect { x: xx, y: yy }
}

fn expand_interval(i: &GeoR1Interval, margin: &f64) -> GeoR1Interval {
    if i.is_empty() {
        return GeoR1Interval::empty();
    }
    GeoR1Interval {
        lo: i.lo - margin,
        hi: i.hi + margin,
    }
}

impl GeoR1Interval {
    fn is_empty(&self) -> bool {
        self.lo > self.hi
    }
}

fn geo_r2_empty_rect() -> GeoR2Rect {
    GeoR2Rect {
        x: GeoR1Interval::empty(),
        y: GeoR1Interval::empty(),
    }
}

impl GeoR1Interval {
    fn empty() -> GeoR1Interval {
        GeoR1Interval { lo: 1.0, hi: 0.0 }
    }
}

