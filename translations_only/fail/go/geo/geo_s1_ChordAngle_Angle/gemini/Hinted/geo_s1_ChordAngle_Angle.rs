
use std::f64::consts::PI;
use std::f64::INFINITY;

#[derive(Debug, Clone, Copy)]
pub struct ChordAngle(pub f64);

#[derive(Debug, Clone, Copy)]
pub struct Angle(pub f64);

impl Angle {
    pub fn from_chord_angle(c: ChordAngle) -> Angle {
        if c.0 < 0.0 {
            return Angle(-1.0 * PI);
        }
        if c.is_infinity() {
            return Angle(INFINITY);
        }
        Angle(2.0 * f64::asin(0.5 * f64::sqrt(c.0)))
    }

    pub fn is_infinity(&self) -> bool {
        self.0.is_infinite()
    }
}

impl ChordAngle {
    pub fn is_infinity(&self) -> bool {
        self.0.is_infinite()
    }
}

pub fn inf_angle() -> Angle {
    Angle(INFINITY)
}
