

use std::io::{Read, BufReader};
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
struct GeoS2Point {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, Clone, Copy)]
struct GeoS2Cap {
    center: GeoS2Point,
    radius: f64,
}

#[derive(Debug, Clone, Copy)]
struct GeoS1ChordAngle(f64);

impl GeoS1ChordAngle {
    fn from_radians(radians: f64) -> Self {
        GeoS1ChordAngle(radians.sin() / (radians.cos() + 1.0))
    }
}

struct GeoS2Decoder<R: Read> {
    r: BufReader<R>,
    err: Option<std::io::Error>,
    buf: [u8; 8],
}

impl<R: Read> GeoS2Decoder<R> {
    fn new(r: R) -> Self {
        GeoS2Decoder {
            r: BufReader::new(r),
            err: None,
            buf: [0; 8],
        }
    }

    fn read_f64(&mut self) -> f64 {
        if let Some(err) = &self.err {
            return 0.0;
        }
        self.r.read_exact(&mut self.buf).unwrap();
        f64::from_le_bytes(self.buf)
    }
}

fn decode_cap<R: Read>(d: &mut GeoS2Decoder<R>) -> GeoS2Cap {
    let center = GeoS2Point {
        x: d.read_f64(),
        y: d.read_f64(),
        z: d.read_f64(),
    };
    let radius = d.read_f64();
    GeoS2Cap { center, radius }
}

