
use std::io::{self, Write};

struct GeoS2Cap {
    center: GeoS2Point,
    radius: GeoS1ChordAngle,
}

struct GeoS2Point {
    geo_r3_vector: GeoR3Vector,
}

struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

type GeoS1ChordAngle = f64;

struct GeoS2Encoder {
    w: Box<dyn Write>,
    err: Option<io::Error>,
}

impl GeoS2Cap {
    fn encode(&self, w: &mut GeoS2Encoder) {
        self.encode_inner(w);
    }

    fn encode_inner(&self, e: &mut GeoS2Encoder) {
        write_float64(e, self.center.geo_r3_vector.x);
        write_float64(e, self.center.geo_r3_vector.y);
        write_float64(e, self.center.geo_r3_vector.z);
        write_float64(e, self.radius as f64);
    }
}

impl GeoS2Encoder {
    fn write_float64(&mut self, x: f64) {
        if self.err.is_none() {
            let mut buf = [0u8; 8];
            LittleEndian::write_f64(&mut buf, x);
            self.err = self.w.write(&buf).err();
        }
    }
}

fn write_float64(e: &mut GeoS2Encoder, x: f64) {
    e.write_float64(x);
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct LittleEndian;

impl LittleEndian {
    fn read_u16(buf: &[u8]) -> u16 {
        u16::from_le_bytes([buf[0], buf[1]])
    }

    fn read_u32(buf: &[u8]) -> u32 {
        u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]])
    }

    fn read_u64(buf: &[u8]) -> u64 {
        u64::from_le_bytes([buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7]])
    }

    fn read_f32(buf: &[u8]) -> f32 {
        f32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]])
    }

    fn read_f64(buf: &[u8]) -> f64 {
        f64::from_le_bytes([buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7]])
    }

    fn write_u16(buf: &mut [u8], n: u16) {
        buf.copy_from_slice(&n.to_le_bytes());
    }

    fn write_u32(buf: &mut [u8], n: u32) {
        buf.copy_from_slice(&n.to_le_bytes());
    }

    fn write_u64(buf: &mut [u8], n: u64) {
        buf.copy_from_slice(&n.to_le_bytes());
    }

    fn write_f32(buf: &mut [u8], n: f32) {
        buf.copy_from_slice(&n.to_le_bytes());
    }

    fn write_f64(buf: &mut [u8], n: f64) {
        buf.copy_from_slice(&n.to_le_bytes());
    }
}
