

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
        if let Some(err) = &self.err {
            return 0.0;
        }

        let mut reader = BufReader::new(&mut self.r);
        let mut buf = [0u8; 8];
        match reader.read_exact(&mut buf) {
            Ok(_) => f64::from_bits(u64::from_le_bytes(buf)),
            Err(err) => {
                self.err = Some(Box::new(err));
                0.0
            }
        }
    }
}

fn decode_geo_s2_cap(cap: &mut GeoS2Cap, decoder: &mut GeoS2Decoder) {
    cap.center.geo_r3_vector.x = decoder.read_float64();
    cap.center.geo_r3_vector.y = decoder.read_float64();
    cap.center.geo_r3_vector.z = decoder.read_float64();
    cap.radius = decoder.read_float64();
}

