

use std::io;

struct GeoS2Cap {
    center: GeoS2Point,
    radius: f64,
}

struct GeoS2Point {
    x: f64,
    y: f64,
    z: f64,  
}

struct GeoS2Decoder {
    r: Box<GeoS2ByteReader>,
    err: Option<io::Error>,
    buf: Option<Box<[u8; 8]>>,
}

struct GeoS2ByteReader {
    // implementation omitted
}

impl GeoS2ByteReader {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        // implementation omitted
        Ok(0)
    }
}

fn decode_cap(cap: &mut GeoS2Cap, decoder: &mut GeoS2Decoder) {
    cap.center.x = read_f64(decoder);
    cap.center.y = read_f64(decoder);
    cap.center.z = read_f64(decoder);
    cap.radius = read_f64(decoder);
}

fn read_f64(decoder: &mut GeoS2Decoder) -> f64 {
    if decoder.err.is_some() {
        return 0.0;
    }
    
    let mut buf = *get_buffer(decoder);
    decoder.r.read(&mut buf).unwrap();
    f64::from_le_bytes(buf)
}

fn get_buffer(decoder: &mut GeoS2Decoder) -> &mut [u8; 8] {
    if decoder.buf.is_none() {
        decoder.buf = Some(Box::new([0u8; 8]));
    }
    decoder.buf.as_mut().unwrap()
}

