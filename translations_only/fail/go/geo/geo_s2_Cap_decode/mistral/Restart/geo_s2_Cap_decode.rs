

use std::io;
use std::io::Cursor;
use std::boxed::Box;
use std::io::Read; // added

#[derive(Default)]
struct GeoS2Cap {
    center: GeoS2Point,
}

#[derive(Default)]
struct GeoS2Point {
    x: f64,
    y: f64,
}

struct GeoS2Decoder {
    reader: Cursor<Vec<u8>>,
}

impl GeoS2Decoder {
    fn read_float6(&mut self) -> f64 {
        let mut buf = [0; 8];
        self.reader.read_exact(&mut buf).unwrap();
        f64::from_le_bytes(buf)
    }
}

fn decode(c: &mut GeoS2Cap, d: &mut GeoS2Decoder) {
    c.center.x = d.read_float6();
}

