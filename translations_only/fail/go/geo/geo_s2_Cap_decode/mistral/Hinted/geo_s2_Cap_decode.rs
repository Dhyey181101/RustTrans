

use std::io::{Read, Error as IoError, ErrorKind, Cursor};
use std::fs::File;
use std::mem;
use std::boxed::Box;

pub struct GeoS2Cap {
    center: GeoS2Point,
    radius: GeoS1ChordAngle,
}

pub struct GeoS2Point {
    x: f64,
    y: f64,
    z: f64,
}

pub struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

pub struct GeoS1ChordAngle(f64);

pub struct GeoS2Decoder {
    r: Box<dyn GeoS2ByteReader>,
    err: IoError,
    buf: Vec<u8>,
}

pub trait GeoS2ByteReader {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, IoError>;
    fn read_u8(&mut self) -> Result<u8, IoError>;
}

impl GeoS2Cap {
    pub fn decode(&mut self, d: &mut GeoS2Decoder) {
        self.center.x = d.read_float64();
        self.center.y = d.read_float64();
        self.center.z = d.read_float64();
        self.radius = GeoS1ChordAngle(d.read_float64());
    }
}

impl GeoS2Decoder {
    pub fn new(r: Box<dyn GeoS2ByteReader>) -> Self {
        GeoS2Decoder {
            r,
            err: IoError::new(ErrorKind::Other, ""),
            buf: Vec::new(),
        }
    }

    fn read_float64(&mut self) -> f64 {
        let mut buf = [0; 8];
        self.r.read(&mut buf).unwrap();
        f64::from_le_bytes(buf)
    }
}

struct FileByteReader {
    file: File,
}

impl GeoS2ByteReader for FileByteReader {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, IoError> {
        self.file.read(buf)
    }

    fn read_u8(&mut self) -> Result<u8, IoError> {
        let mut buf = [0];
        self.file.read_exact(&mut buf)?;
        Ok(buf[0])
    }
}

