
use std::io::{Error, Write};

pub struct GeoS2Cap {
    pub center: GeoS2Point,
    pub radius: GeoS1ChordAngle,
}

pub struct GeoS2Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct GeoS1ChordAngle(pub f64);

pub struct GeoS2Encoder {
    pub w: Box<dyn Write>,
    pub err: Option<Error>,
}

impl GeoS2Cap {
    pub fn encode(&self, w: &mut GeoS2Encoder) {
        w.write_f64(self.center.x);
        w.write_f64(self.center.y);
        w.write_f64(self.center.z);
        w.write_f64(self.radius.0);
    }
}

impl GeoS2Encoder {
    pub fn write_f64(&mut self, x: f64) {
        if self.err.is_some() {
            return;
        }
        self.err = match self.w.write_all(&x.to_le_bytes()) {
            Ok(_) => None,
            Err(e) => Some(e),
        };
    }
}
