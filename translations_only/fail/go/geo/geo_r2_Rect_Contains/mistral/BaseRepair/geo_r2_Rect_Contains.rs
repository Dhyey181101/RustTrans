

use std::ops::*;

struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

struct GeoR2Rect {
    x: GeoR1Interval,
    y: GeoR1Interval,
}

fn contains(g1: &GeoR1Interval, g2: &GeoR1Interval) -> bool {
    if is_empty(g2) {
        return true;
    }
    g1.lo <= g2.lo && g2.hi <= g1.hi
}

fn is_empty(g1: &GeoR1Interval) -> bool {
    g1.lo > g1.hi
}

fn contains_rect(g1: &GeoR2Rect, g2: &GeoR2Rect) -> bool {
    contains(&g1.x, &g2.x) && contains(&g1.y, &g2.y)
}

impl GeoR1Interval {
    fn new(lo: f64, hi: f64) -> Self {
        GeoR1Interval { lo, hi }
    }
}

impl GeoR2Rect {
    fn new(x: GeoR1Interval, y: GeoR1Interval) -> Self {
        GeoR2Rect { x, y }
    }
}

