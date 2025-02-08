
use std::io::{self, Write};

struct GeoS2CellID(u64);

struct GeoS2Encoder {
    w: Box<dyn Write>,
    err: Option<io::Error>,
}

fn encode(cell_id: &GeoS2CellID, w: Box<dyn Write>) -> io::Result<()> {
    let mut e = GeoS2Encoder {
        w,
        err: None,
    };
    encode_inner(cell_id, &mut e);
    match e.err {
        Some(err) => Err(err),
        None => Ok(()),
    }
}

fn encode_inner(cell_id: &GeoS2CellID, e: &mut GeoS2Encoder) {
    write_uint64(e, cell_id.0);
}

fn write_uint64(e: &mut GeoS2Encoder, x: u64) {
    if e.err.is_some() {
        return;
    }
    let mut buf = [0u8; 8];
    buf[0] = x as u8;
    buf[1] = (x >> 8) as u8;
    buf[2] = (x >> 16) as u8;
    buf[3] = (x >> 24) as u8;
    buf[4] = (x >> 32) as u8;
    buf[5] = (x >> 40) as u8;
    buf[6] = (x >> 48) as u8;
    buf[7] = (x >> 56) as u8;
    e.err = e.w.write_all(&buf).err();
}

fn main() {}
