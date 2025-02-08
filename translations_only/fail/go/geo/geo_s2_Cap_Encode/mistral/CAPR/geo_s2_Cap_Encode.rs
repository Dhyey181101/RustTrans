

use std::boxed::Box;
use std::convert::TryInto;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Write;
use std::mem::size_of;

struct GeoS2Cap {
    center: GeoS2Point,
    radius: GeoS1ChordAngle,
}

struct GeoS2Point {
    x: f64,
    y: f64,
    z: f64,
}

struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

type GeoS1ChordAngle = f64;

struct GeoS2Encoder {
    w: Box<File>,
}

impl GeoS2Cap {
    fn encode(&self, w: &mut File) -> Result<(), Box<dyn Error>> {
        let mut e = GeoS2Encoder {
            w: Box::new(w.try_clone()?),
        };
        self.encode_inner(&mut e);
        Ok(())
    }

    fn encode_inner(&self, e: &mut GeoS2Encoder) {
        e.write_float64(self.center.x);
        e.write_float64(self.center.y);
        e.write_float64(self.center.z);
        e.write_float64(self.radius.to_radians());
    }
}

impl GeoS2Encoder {
    fn new(w: File) -> Self {
        GeoS2Encoder { w: Box::new(w) }
    }

    fn write_float64(&mut self, f: f64) {
        let mut buf = [0; size_of::<f64>()];
        buf.copy_from_slice(&f.to_le_bytes());
        self.w.write_all(&buf).unwrap();
    }

    fn write_float32(&mut self, f: f32) {
        let mut buf = [0; size_of::<f32>()];
        buf.copy_from_slice(&f.to_le_bytes());
        self.w.write_all(&buf).unwrap();
    }

    fn write_uint32(&mut self, i: u32) {
        let mut buf = [0; size_of::<u32>()];
        buf.copy_from_slice(&i.to_le_bytes());
        self.w.write_all(&buf).unwrap();
    }
}

