
use std::io;

struct GeoS2Cap {
    center: GeoS2Point,
    radius: f64,
}

struct GeoS2Point {
    geo_r3_vector: GeoR3Vector,
}

struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

struct GeoS2Decoder {
    r: Box<dyn io::Read>,
    buf: Box<[u8]>,
}

fn decode_geo_s2_cap(cap: &mut GeoS2Cap, decoder: &mut GeoS2Decoder) {
    cap.center.geo_r3_vector.x = read_float64(&mut decoder.r, &mut decoder.buf);
    cap.center.geo_r3_vector.y = read_float64(&mut decoder.r, &mut decoder.buf);
    cap.center.geo_r3_vector.z = read_float64(&mut decoder.r, &mut decoder.buf);
    cap.radius = read_float64(&mut decoder.r, &mut decoder.buf);
}

fn read_float64(r: &mut Box<dyn io::Read>, buf: &mut Box<[u8]>) -> f64 {
    if buf.is_empty() {
        *buf = Box::new([0; 8]);
    }

    if let Err(err) = r.read_exact(&mut buf[..]) {
        return 0.0;
    }

    f64::from_bits(u64::from_le_bytes(buf[..].try_into().unwrap()))
}
