
use std::io::{self, Write};

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

struct GeoS2Encoder<'a> {
    w: &'a mut dyn Write,
    err: Option<io::Error>,
}

fn encode(cap: &GeoS2Cap, encoder: &mut GeoS2Encoder) {
    write_float64(encoder, cap.center.vector.x);
    write_float64(encoder, cap.center.vector.y);
    write_float64(encoder, cap.center.vector.z);
    write_float64(encoder, cap.radius as f64);
}

fn write_float64(encoder: &mut GeoS2Encoder, x: f64) {
    if encoder.err.is_some() {
        return;
    }
    let mut buf = [0u8; 8];
    buf.copy_from_slice(&x.to_le_bytes());
    encoder.err = encoder.w.write_all(&buf).err();
}

impl GeoS2Cap {
    fn encode(&self, w: &mut dyn Write) -> io::Result<()> {
        let mut encoder = GeoS2Encoder { w, err: None };
        encode(self, &mut encoder);
        match encoder.err {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }
}
