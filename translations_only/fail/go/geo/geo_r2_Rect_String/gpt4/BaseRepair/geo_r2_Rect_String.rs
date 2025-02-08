
#[derive(Debug)]
struct GeoR2Rect {
    x: GeoR1Interval,
    y: GeoR1Interval,
}

#[derive(Debug)]
struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

#[derive(Debug)]
struct GeoR2Point {
    x: f64,
    y: f64,
}

fn lo(r: &GeoR2Rect) -> Box<GeoR2Point> {
    Box::new(GeoR2Point {
        x: r.x.lo,
        y: r.y.lo,
    })
}

fn hi(r: &GeoR2Rect) -> Box<GeoR2Point> {
    Box::new(GeoR2Point {
        x: r.x.hi,
        y: r.y.hi,
    })
}

fn to_string(r: &GeoR2Rect) -> String {
    format!("[Lo{:?}, Hi{:?}]", lo(r), hi(r))
}
