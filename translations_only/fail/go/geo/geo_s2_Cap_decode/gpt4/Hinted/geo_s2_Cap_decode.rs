
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

type GeoS1ChordAngle = f64;

struct GeoS2Decoder<'a> {
    r: Box<dyn Read + 'a>,
    err: Option<io::Error>,
    buf: Vec<u8>,
}

impl GeoS2Cap {
    fn decode(&mut self, d: &mut GeoS2Decoder) {
        self.center.vector.x = read_float64(d);
        self.center.vector.y = read_float64(d);
        self.center.vector.z = read_float64(d);
        self.radius = read_float64(d);
    }
}

fn read_float64(d: &mut GeoS2Decoder) -> f64 {
    if d.err.is_some() {
        return 0.0;
    }
    let mut buf = &mut d.buf[..];
    match d.r.read_exact(&mut buf) {
        Ok(_) => f64::from_le_bytes(buf.try_into().unwrap()),
        Err(e) => {
            d.err = Some(e);
            0.0
        }
    }
}
