
use std::io::{Read, BufReader};
use std::f64::consts::PI;

#[derive(Debug)]
pub struct GeoS2Cap {
    pub center: GeoS2Point,
    pub radius: GeoS1ChordAngle,
}

impl GeoS2Cap {
    pub fn decode(d: &mut GeoS2Decoder) -> Self {
        Self {
            center: GeoS2Point {
                x: d.read_float64(),
                y: d.read_float64(),
                z: d.read_float64(),
            },
            radius: GeoS1ChordAngle(d.read_float64()),
        }
    }
}

#[derive(Debug)]
pub struct GeoS2Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug)]
pub struct GeoS1ChordAngle(pub f64);

pub struct GeoS2Decoder {
    r: BufReader<Box<dyn Read>>,
    err: Option<std::io::Error>,
    buf: [u8; 8],
}

impl GeoS2Decoder {
    pub fn new(r: Box<dyn Read>) -> Self {
        Self {
            r: BufReader::new(r),
            err: None,
            buf: [0; 8],
        }
    }

    pub fn read_float64(&mut self) -> f64 {
        if let Some(err) = &self.err {
            return 0.0;
        }
        self.r.read_exact(&mut self.buf).unwrap();
        f64::from_le_bytes(self.buf)
    }
}
