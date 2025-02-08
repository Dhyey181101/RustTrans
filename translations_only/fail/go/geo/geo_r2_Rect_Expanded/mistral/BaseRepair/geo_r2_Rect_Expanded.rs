
type GeoR2Rect = (GeoR1Interval, GeoR1Interval);
type GeoR1Interval = (f64, f64);
type GeoR2Point = (f64, f64);

fn geo_r2_empty_rect() -> GeoR2Rect {
    (geo_r1_empty_interval(), geo_r1_empty_interval())
}

fn geo_r1_empty_interval() -> GeoR1Interval {
    (1.0, 0.0)
}

fn geo_r2_expanded(r: GeoR2Rect, margin: GeoR2Point) -> GeoR2Rect {
    let xx = geo_r1_expanded(r.0, margin.0);
    let yy = geo_r1_expanded(r.1, margin.1);
    if xx.0 > xx.1 || yy.0 > yy.1 {
        geo_r2_empty_rect()
    } else {
        (xx, yy)
    }
}

fn geo_r1_expanded(i: GeoR1Interval, margin: f64) -> GeoR1Interval {
    if i.0 > i.1 {
        geo_r1_empty_interval()
    } else {
        (i.0 - margin, i.1 + margin)
    }
}

fn geo_r1_is_empty(i: GeoR1Interval) -> bool {
    i.0 > i.1
}
