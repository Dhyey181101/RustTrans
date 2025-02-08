
const GEO_S1_MAX_LENGTH2: f64 = 4.0;

struct GeoS1ChordAngle(f64);

impl GeoS1ChordAngle {
    fn new(angle: f64) -> Box<Self> {
        Box::new(Self(angle))
    }
}

fn is_valid(c: &GeoS1ChordAngle) -> bool {
    (c.0 >= 0.0 && c.0 <= GEO_S1_MAX_LENGTH2) || is_special(c)
}

fn is_special(c: &GeoS1ChordAngle) -> bool {
    c.0 < 0.0 || is_infinity(c)
}

fn is_infinity(c: &GeoS1ChordAngle) -> bool {
    c.0.is_infinite() && c.0.is_sign_positive()
}
