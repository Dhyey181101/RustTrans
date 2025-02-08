
const GEO_S1_MAX_LENGTH2: f64 = 4.0;

struct GeoS1ChordAngle(f64);

impl GeoS1ChordAngle {
    fn expanded(&self, e: f64) -> Box<GeoS1ChordAngle> {
        if self.is_special() {
            Box::new(GeoS1ChordAngle(self.0))
        } else {
            Box::new(GeoS1ChordAngle(f64::max(0.0, f64::min(GEO_S1_MAX_LENGTH2, self.0 + e))))
        }
    }

    fn is_special(&self) -> bool {
        self.0 < 0.0 || self.is_infinity()
    }

    fn is_infinity(&self) -> bool {
        self.0.is_infinite()
    }
}
