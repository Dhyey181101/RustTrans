
use std::io::{Read, Result};

struct GeoS2Cap {
    center: GeoS2Point,
    radius: GeoS1ChordAngle,
}

struct GeoS2Point {
    x: f64,
    y: f64,
    z: f64,  
}

struct GeoS2Decoder {
    r: Box<dyn Read>,
    err: Option<std::io::Error>,
    buf: Option<Box<[u8; 8]>>,
}

struct GeoS1ChordAngle(f64);

fn read_from_reader(r: &mut Box<dyn Read>, buf: &mut [u8]) -> Result<usize> {
    r.read(buf)
}

fn decode_cap(cap: &mut GeoS2Cap, decoder: &mut GeoS2Decoder) {
    cap.center.x = read_f64(decoder);
    cap.center.y = read_f64(decoder);
    cap.center.z = read_f64(decoder);
    cap.radius = GeoS1ChordAngle(read_f64(decoder));
}

fn read_f64(decoder: &mut GeoS2Decoder) -> f64 {
    if decoder.err.is_some() {
        return 0.0;
    }
    let mut buf = *decoder.buffer();
    read_from_reader(&mut decoder.r, &mut buf).unwrap();
    f64::from_le_bytes(buf.try_into().unwrap())
}

impl GeoS2Decoder {
    fn buffer(&mut self) -> &mut [u8; 8] {
        if self.buf.is_none() {
            self.buf = Some(Box::new([0u8; 8]));
        }
        self.buf.as_mut().unwrap()
    }
}

