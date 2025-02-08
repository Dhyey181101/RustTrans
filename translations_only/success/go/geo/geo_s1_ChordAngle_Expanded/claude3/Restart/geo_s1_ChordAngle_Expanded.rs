
use std::f64;

const GEO_S1_MAX_LENGTH2: f64 = 4.0;

fn expanded(c: GeoS1ChordAngle, e: f64) -> GeoS1ChordAngle {
    // If the angle is special, don't change it. Otherwise clamp it to the valid range.
    if is_special(c) {
        return c;
    }
    GeoS1ChordAngle(f64::max(0.0, f64::min(GEO_S1_MAX_LENGTH2, c.0 + e)))
}

fn is_special(c: GeoS1ChordAngle) -> bool {
    c.0 < 0.0 || is_infinity(c)
}

fn is_infinity(c: GeoS1ChordAngle) -> bool {
    c.0.is_infinite()
}

#[derive(Copy, Clone)]
struct GeoS1ChordAngle(f64);
