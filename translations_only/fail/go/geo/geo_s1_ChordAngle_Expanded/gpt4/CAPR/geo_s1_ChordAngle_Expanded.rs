
const GEO_S1_MAX_LENGTH2: f64 = 4.0;

struct GeoS1ChordAngle(f64);

impl GeoS1ChordAngle {
    fn expanded(&self, e: f64) -> Box<GeoS1ChordAngle> {
        if self.is_special() {
            return Box::new(GeoS1ChordAngle(self.0));
        }
        Box::new(GeoS1ChordAngle(self.0.max(0.0).min(GEO_S1_MAX_LENGTH2) + e))
    }

    fn is_special(&self) -> bool {
        self.0 < 0.0 || self.is_infinity()
    }

    fn is_infinity(&self) -> bool {
        self.0.is_infinite()
    }
}
