

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
        self.encode_inner(w);
    }

    fn encode_inner(&self, e: &mut GeoS2Encoder) {
        e.write_float64(self.center.geo_r3_vector.x);
        e.write_float64(self.center.geo_r3_vector.y);
        e.write_float64(self.center.geo_r3_vector.z);
        e.write_float64(self.radius as f64);
    }
}

impl GeoS2Encoder {
    fn write_float64(&mut self, x: f64) {
        if self.err.is_none() {
            self.err = binary_write_float64(&mut self.w, x).err();
        }
    }
}

fn binary_write_float64(w: &mut Box<dyn Write>, x: f64) -> io::Result<()> {
    let bytes = x.to_le_bytes();
    w.write_all(&bytes)
}

