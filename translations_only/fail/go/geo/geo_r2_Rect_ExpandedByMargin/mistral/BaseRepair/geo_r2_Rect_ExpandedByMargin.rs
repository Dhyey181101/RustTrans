
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
    fn expanded_by_margin(&self, margin: f64) -> GeoR2Rect {
        self.expanded(GeoR2Point {
            x: margin,
            y: margin,
        })
    }

    fn expanded(&self, margin: GeoR2Point) -> GeoR2Rect {
        let xx = self.x.expanded(margin.x);
        let yy = self.y.expanded(margin.y);
        if xx.is_empty() || yy.is_empty() {
            GeoR2EmptyRect
        } else {
            GeoR2Rect { x: xx, y: yy }
        }
    }
}

impl GeoR1Interval {
    fn expanded(&self, margin: f64) -> GeoR1Interval {
        if self.is_empty() {
            GeoR1EmptyInterval
        } else {
            GeoR1Interval {
                lo: self.lo - margin,
                hi: self.hi + margin,
            }
        }
    }

    fn is_empty(&self) -> bool {
        self.lo > self.hi
    }
}

const GeoR2EmptyRect: GeoR2Rect = GeoR2Rect {
    x: GeoR1EmptyInterval,
    y: GeoR1EmptyInterval,
};

const GeoR1EmptyInterval: GeoR1Interval = GeoR1Interval { lo: 1.0, hi: 0.0 };
