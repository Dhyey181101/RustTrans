
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

impl GeoS2Cap {
    fn encode(&self, w: &mut dyn Write) -> io::Result<()> {
        let mut e = GeoS2Encoder { w, err: None };
        encode_cap(self, &mut e);
        match e.err {
            Some(err) => Err(err),
            None => Ok(()),
        }
    }
}

fn encode_cap(cap: &GeoS2Cap, e: &mut GeoS2Encoder) {
    write_float64(&mut e.w, cap.center.vector.x, &mut e.err);
    write_float64(&mut e.w, cap.center.vector.y, &mut e.err);
    write_float64(&mut e.w, cap.center.vector.z, &mut e.err);
    write_float64(&mut e.w, cap.radius, &mut e.err);
}

fn write_float64(w: &mut dyn Write, x: f64, err: &mut Option<io::Error>) {
    if err.is_some() {
        return;
    }
    *err = w.write_all(&x.to_le_bytes()).err();
}
