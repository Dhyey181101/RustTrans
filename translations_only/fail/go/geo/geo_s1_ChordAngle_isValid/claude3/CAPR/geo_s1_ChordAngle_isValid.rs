
use std::f64;

const GEO_S1_MAX_LENGTH2: f64 = 4.0;

fn is_valid(c: GeoS1ChordAngle) -> bool {
    (c >= 0.0 && c <= GEO_S1_MAX_LENGTH2) || is_special(c)
}

fn is_special(c: GeoS1ChordAngle) -> bool {
    c < 0.0 || is_infinity(c)
}

fn is_infinity(c: GeoS1ChordAngle) -> bool {
    f64::is_infinite(c)
}

type GeoS1ChordAngle = f64;
