
struct GeoR2Rect {
    x: GeoR1Interval,
    y: GeoR1Interval,
}

struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

#[derive(Debug)]
struct GeoR2Point {
    x: f64,
    y: f64,
}

fn geo_r2_rect_string(r: &GeoR2Rect) -> String {
    format!("[Lo{:?}, Hi{:?}]", Box::new(geo_r2_rect_lo(r)), Box::new(geo_r2_rect_hi(r)))
}

fn geo_r2_rect_lo(r: &GeoR2Rect) -> GeoR2Point {
    GeoR2Point {
        x: r.x.lo,
        y: r.y.lo,
    }
}

fn geo_r2_rect_hi(r: &GeoR2Rect) -> GeoR2Point {
    GeoR2Point {
        x: r.x.hi,
        y: r.y.hi,
    }
}
