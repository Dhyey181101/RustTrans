
use std::cmp::Ordering;

#[derive(Clone)]
struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

struct GeoR2Point {
    x: f64,
    y: f64,  
}

struct GeoR2Rect {
    x: Box<GeoR1Interval>,
    y: Box<GeoR1Interval>,
}

fn geo_r1_empty_interval() -> GeoR1Interval {
    GeoR1Interval { lo: 1.0, hi: 0.0 }
}

fn geo_r2_empty_rect() -> GeoR2Rect {
    GeoR2Rect {
        x: Box::new(geo_r1_empty_interval()),
        y: Box::new(geo_r1_empty_interval()),
    }
}

fn geo_r1_interval_is_empty(interval: &GeoR1Interval) -> bool {
    interval.lo > interval.hi
}

fn geo_r1_interval_expanded(interval: &GeoR1Interval, margin: f64) -> GeoR1Interval {
    if geo_r1_interval_is_empty(interval) {
        interval.clone()
    } else {
        GeoR1Interval {
            lo: interval.lo - margin,
            hi: interval.hi + margin,
        }
    }
}

fn geo_r2_rect_expanded(rect: &GeoR2Rect, margin: GeoR2Point) -> GeoR2Rect {
    let x = geo_r1_interval_expanded(&*rect.x, margin.x);
    let y = geo_r1_interval_expanded(&*rect.y, margin.y);
    if geo_r1_interval_is_empty(&x) || geo_r1_interval_is_empty(&y) {
        geo_r2_empty_rect()
    } else {
        GeoR2Rect { 
            x: Box::new(x),
            y: Box::new(y),
        }
    }
}

fn geo_r2_rect_expanded_by_margin(rect: &GeoR2Rect, margin: f64) -> GeoR2Rect {
    geo_r2_rect_expanded(rect, GeoR2Point { x: margin, y: margin })
}

