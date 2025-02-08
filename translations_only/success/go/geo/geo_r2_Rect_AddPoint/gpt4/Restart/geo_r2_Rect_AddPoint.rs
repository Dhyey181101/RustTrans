
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

impl GeoR2Rect {
    fn add_point(&self, p: GeoR2Point) -> GeoR2Rect {
        GeoR2Rect {
            x: add_point_interval(&self.x, p.x),
            y: add_point_interval(&self.y, p.y),
        }
    }
}

fn add_point_interval(interval: &GeoR1Interval, p: f64) -> GeoR1Interval {
    if interval.is_empty() {
        return GeoR1Interval { lo: p, hi: p };
    }
    if p < interval.lo {
        return GeoR1Interval { lo: p, hi: interval.hi };
    }
    if p > interval.hi {
        return GeoR1Interval { lo: interval.lo, hi: p };
    }
    GeoR1Interval { lo: interval.lo, hi: interval.hi }
}

impl GeoR1Interval {
    fn is_empty(&self) -> bool {
        self.lo > self.hi
    }
}

fn main() {}
