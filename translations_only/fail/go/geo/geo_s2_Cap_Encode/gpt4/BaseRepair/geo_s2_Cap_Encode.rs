
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

fn encode_geo_s2_cap(cap: &GeoS2Cap, w: &mut dyn Write) -> io::Result<()> {
    let mut e = GeoS2Encoder { w, err: None };
    encode_internal(&cap.center.vector.x, &mut e);
    encode_internal(&cap.center.vector.y, &mut e);
    encode_internal(&cap.center.vector.z, &mut e);
    encode_internal(&cap.radius, &mut e);
    match e.err {
        Some(err) => Err(err),
        None => Ok(()),
    }
}

fn encode_internal(x: &f64, e: &mut GeoS2Encoder) {
    if e.err.is_some() {
        return;
    }
    let mut buf = [0; 8];
    LittleEndian::write_f64(&mut buf, *x);
    e.err = e.w.write_all(&buf).err();
}

fn little_endian_write_f64(buf: &mut [u8; 8], x: f64) {
    buf.copy_from_slice(&x.to_le_bytes());
}

struct LittleEndian;

impl LittleEndian {
    fn write_f64(buf: &mut [u8; 8], x: f64) {
        little_endian_write_f64(buf, x);
    }
}
