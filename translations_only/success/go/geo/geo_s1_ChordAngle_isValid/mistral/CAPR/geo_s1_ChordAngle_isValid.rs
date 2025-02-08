
pub const GEO_S1_MAX_LENGTH2: f64 = 4.0;

#[derive(Copy, Clone)]
pub struct GeoS1ChordAngle(f64);

impl GeoS1ChordAngle {
    pub fn is_valid(&self) -> bool {
        (self.0 >= 0.0 && self.0 <= GEO_S1_MAX_LENGTH2) || self.is_special()
    }

    pub fn is_special(&self) -> bool {
        self.0 < 0.0 || self.is_infinity()
    }

    pub fn is_infinity(&self) -> bool {
        self.0.is_infinite()
    }
}
