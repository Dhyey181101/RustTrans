
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

fn decode_geo_s2_cap(cap: &mut GeoS2Cap, d: &mut GeoS2Decoder) {
    cap.center.vector.x = read_float64(d);
    cap.center.vector.y = read_float64(d);
    cap.center.vector.z = read_float64(d);
    cap.radius = read_float64(d);
}

fn read_float64(d: &mut GeoS2Decoder) -> f64 {
    if d.err.is_some() {
        return 0.0;
    }
    let buf = &mut d.buf;
    buf.resize(8, 0);
    match d.r.read_exact(buf) {
        Ok(_) => {
            let temp = buf.as_slice();
            f64::from_le_bytes(temp.try_into().unwrap())
        },
        Err(e) => {
            d.err = Some(e);
            0.0
        }
    }
}

fn main() {}
