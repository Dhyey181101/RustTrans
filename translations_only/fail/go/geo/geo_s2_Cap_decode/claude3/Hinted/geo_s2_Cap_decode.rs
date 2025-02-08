

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
    fn decode_geo_s2_cap(&mut self, cap: &mut GeoS2Cap) {
        cap.center.geo_r3_vector.x = self.read_float64();
        cap.center.geo_r3_vector.y = self.read_float64();
        cap.center.geo_r3_vector.z = self.read_float64();
        cap.radius = self.read_float64();
    }

    fn read_float64(&mut self) -> f64 {
        if let Some(err) = &self.err {
            return 0.0;
        }
        match self.r.read_exact(&mut *self.buf) {
            Ok(_) => f64::from_bits(u64::from_le_bytes(*self.buf)),
            Err(e) => {
                self.err = Some(Box::new(e));
                0.0
            }
        }
    }
}

