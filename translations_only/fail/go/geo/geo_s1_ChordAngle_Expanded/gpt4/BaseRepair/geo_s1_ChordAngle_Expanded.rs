
const GEO_S1_MAX_LENGTH2: f64 = 4.0;

struct GeoS1ChordAngle(Box<f64>);

fn expanded(angle: &GeoS1ChordAngle, e: f64) -> GeoS1ChordAngle {
    if is_special(angle) {
        return GeoS1ChordAngle(Box::new(*angle.0));
    }
    GeoS1ChordAngle(Box::new(angle.0.max(0.0).min(GEO_S1_MAX_LENGTH2) + e))
}

fn is_special(angle: &GeoS1ChordAngle) -> bool {
    *angle.0 < 0.0 || is_infinity(angle)
}

fn is_infinity(angle: &GeoS1ChordAngle) -> bool {
    angle.0.is_infinite()
}
