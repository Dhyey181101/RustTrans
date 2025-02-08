
const GEO_S1_MAX_LENGTH2: f64 = 4.0;

struct GeoS1ChordAngle(f64);

impl GeoS1ChordAngle {
    fn new(angle: f64) -> Box<Self> {
        Box::new(GeoS1ChordAngle(angle))
    }

    fn is_valid(&self) -> bool {
        (self.0 >= 0.0 && self.0 <= GEO_S1_MAX_LENGTH2) || self.is_special()
    }

    fn is_special(&self) -> bool {
        self.0 < 0.0 || self.is_infinity()
    }

    fn is_infinity(&self) -> bool {
        self.0.is_infinite()
    }
}
