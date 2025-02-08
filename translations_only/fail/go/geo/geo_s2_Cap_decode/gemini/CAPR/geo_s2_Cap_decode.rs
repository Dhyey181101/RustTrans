
use std::io::{Read, Seek, SeekFrom};

pub struct GeoS2Cap {
    pub center: GeoS2Point,
    pub radius: GeoS1ChordAngle,
}

pub struct GeoS2Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct GeoS1ChordAngle(pub f64);

pub struct GeoS2Decoder<R: Read + Seek> {
    r: R,
    err: Option<std::io::Error>,
    buf: [u8; 8],
}

impl<R: Read + Seek> GeoS2Decoder<R> {
    pub fn new(r: R) -> Self {
        Self { r, err: None, buf: [0; 8] }
    }

    pub fn read_float64(&mut self) -> f64 {
        if let Some(err) = &self.err {
            return 0.0;
        }

        self.r.read_exact(&mut self.buf).unwrap();
        f64::from_le_bytes(self.buf)
    }

    pub fn buffer(&mut self) -> &mut [u8] {
        &mut self.buf
    }
}

impl GeoS2Cap {
    pub fn decode<R: Read + Seek>(&mut self, d: &mut GeoS2Decoder<R>) {
        self.center.x = d.read_float64();
        self.center.y = d.read_float64();
        self.center.z = d.read_float64();
        self.radius = GeoS1ChordAngle(d.read_float64());
    }
}
