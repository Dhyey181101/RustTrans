
use std::f64::INFINITY;

const GEO_S1_MAXLENGTH2: f64 = 4.0;

struct GeoS1ChordAngle(f64);

fn is_valid(c: GeoS1ChordAngle) -> bool {
    (c.0 >= 0.0 && c.0 <= GEO_S1_MAXLENGTH2) || is_special(c)
}

fn is_special(c: GeoS1ChordAngle) -> bool {
    c.0 < 0.0 || is_infinity(c)  
}

fn is_infinity(c: GeoS1ChordAngle) -> bool {
    c.0.is_infinite()
}
