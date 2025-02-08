
use std::f64::{self, INFINITY};

const GEO_S1_MAXLENGTH2: f64 = 4.0;

struct GeoS1ChordAngle(f64);

fn expanded(mut c: Box<GeoS1ChordAngle>, e: f64) -> Box<GeoS1ChordAngle> {
    if is_special(&c) {
        c
    } else {
        c.0 = f64::max(0.0, f64::min(GEO_S1_MAXLENGTH2, c.0 + e));
        c
    }
}

fn is_special(c: &GeoS1ChordAngle) -> bool {
    c.0 < 0.0 || is_infinity(c)  
}

fn is_infinity(c: &GeoS1ChordAngle) -> bool {
    c.0.is_infinite()
}

