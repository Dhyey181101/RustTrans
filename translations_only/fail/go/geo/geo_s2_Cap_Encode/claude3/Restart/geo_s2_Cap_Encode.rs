

use std::io::{self, Write};

struct GeoS2Cap {
    center: GeoS2Point,
    radius: GeoS1ChordAngle,
}

struct GeoS2Point {
    geo_r3_vector: GeoR3Vector,
}

struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

type GeoS1ChordAngle = f64;

struct GeoS2Encoder {
    w: Box<dyn Write>,
    err: Option<io::Error>,
}

impl GeoS2Cap {
    fn encode(&self, w: &mut GeoS2Encoder) {
        write_float64(w, self.center.geo_r3_vector.x);
        write_float64(w, self.center.geo_r3_vector.y);
        write_float64(w, self.center.geo_r3_vector.z);
        write_float64(w, self.radius);
    }
}

impl GeoS2Encoder {
    fn write_float64(&mut self, x: f64) {
        if let Some(err) = self.err.take() {
            self.err = Some(err);
            return;
        }
        let mut buf = [0u8; 8];
        let bytes = x.to_le_bytes();
        buf.copy_from_slice(&bytes);
        if let Err(err) = self.w.write(&buf) {
            self.err = Some(err);
        }
    }
}

fn write_float64(w: &mut GeoS2Encoder, x: f64) {
    w.write_float64(x);
}

