
use std::f64::INFINITY;

#[derive(Copy, Clone)]
struct GeoS1Angle(f64);

fn new_geo_s1_angle(v: f64) -> GeoS1Angle {
    GeoS1Angle(v)  
}

type GeoS1ChordAngle = f64;

static GEO_S1_RADIAN: GeoS1Angle = GeoS1Angle(1.0);

fn angle(c: GeoS1ChordAngle) -> GeoS1Angle {
    if c < 0.0 {
        neg_geo_s1_angle(GEO_S1_RADIAN)
    } else if is_infinity(c) {
        geo_s1_inf_angle()
    } else {
        new_geo_s1_angle(2.0 * (0.5 * c.sqrt()).asin())
    }
}

fn is_infinity(c: GeoS1ChordAngle) -> bool {
    c.is_infinite()  
}

fn geo_s1_inf_angle() -> GeoS1Angle {
    new_geo_s1_angle(INFINITY)
}

fn neg_geo_s1_angle(a: GeoS1Angle) -> GeoS1Angle {
    new_geo_s1_angle(-a.0) 
}

