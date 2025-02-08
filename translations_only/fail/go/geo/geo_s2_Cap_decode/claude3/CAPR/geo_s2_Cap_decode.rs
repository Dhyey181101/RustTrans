
use std::io::{Read, BufReader};

struct GeoS2Cap {
    center: GeoS2Point,
    radius: f64,
}

struct GeoS2Point {
    geo_r3_vector: GeoR3Vector,
}

struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

struct GeoS2Decoder {
    r: Box<dyn Read>,
    err: Option<Box<std::io::Error>>,
    buf: Box<[u8; 8]>,
}

impl GeoS2Decoder {
    fn read_float64(&mut self) -> f64 {
        if let Some(err) = self.err.as_ref() {
            return 0.0;
        }
        let mut buf = [0u8; 8];
        match self.r.read_exact(&mut buf) {
            Ok(_) => f64::from_bits(u64::from_le_bytes(buf)),
            Err(e) => {
                self.err = Some(Box::new(e));
                0.0
            }
        }
    }
}

impl GeoS2Cap {
    fn decode(&mut self, d: &mut GeoS2Decoder) {
        self.center.geo_r3_vector.x = d.read_float64();
        self.center.geo_r3_vector.y = d.read_float64();
        self.center.geo_r3_vector.z = d.read_float64();
        self.radius = d.read_float64();
    }
}
