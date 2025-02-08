
use std::io::{self, Read};

struct GeoS2Cap {
    center: GeoS2Point,
    radius: GeoS1ChordAngle,
}

struct GeoS2Point {
    vector: GeoR3Vector,
}

struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

struct GeoS2Decoder {
    reader: Box<dyn Read>,
    err: Option<io::Error>,
}

type GeoS1ChordAngle = f64;

fn decode_cap(cap: &mut GeoS2Cap, d: &mut GeoS2Decoder) {
    cap.center.vector.x = read_float64(d);
    cap.center.vector.y = read_float64(d);
    cap.center.vector.z = read_float64(d);
    cap.radius = read_float64(d);
}

fn read_float64(d: &mut GeoS2Decoder) -> f64 {
    if d.err.is_some() {
        return 0.0;
    }
    let mut buf = [0u8; 8];
    match d.reader.read_exact(&mut buf) {
        Ok(_) => f64::from_le_bytes(buf),
        Err(e) => {
            d.err = Some(e);
            0.0
        },
    }
}

fn main() {}
