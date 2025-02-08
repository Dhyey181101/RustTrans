
use std::boxed::Box;

#[derive(Clone)]
struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

impl GeoR1Interval {
    fn is_empty(&self) -> bool {
        self.lo > self.hi
    }
}

struct GeoR2Rect {
    x: Box<GeoR1Interval>,
    y: Box<GeoR1Interval>,
}

fn expanded_rect(rect: &GeoR2Rect, margin: &GeoR2Point) -> GeoR2Rect {
    let x = expanded_interval(&rect.x, margin.x);
    let y = expanded_interval(&rect.y, margin.y);
    if x.is_empty() || y.is_empty() {
        empty_rect()
    } else {
        GeoR2Rect {
            x: Box::new(x),
            y: Box::new(y),
        }
    }
}

fn expanded_interval(interval: &GeoR1Interval, margin: f64) -> GeoR1Interval {
    if interval.is_empty() {
        interval.clone()
    } else {
        GeoR1Interval {
            lo: interval.lo - margin,
            hi: interval.hi + margin,
        }
    }
}

fn empty_rect() -> GeoR2Rect {
    GeoR2Rect {
        x: Box::new(empty_interval()),
        y: Box::new(empty_interval()),
    }
}

fn empty_interval() -> GeoR1Interval {
    GeoR1Interval { lo: 1.0, hi: 0.0 } 
}

struct GeoR2Point {
    x: f64,
    y: f64,
}

