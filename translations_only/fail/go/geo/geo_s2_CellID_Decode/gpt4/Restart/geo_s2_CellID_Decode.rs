
use std::io::{self, BufReader, Read};

struct GeoS2CellID(u64);

struct GeoS2Decoder<R: Read> {
    r: R,
    err: Option<io::Error>,
}

fn geo_s2_as_byte_reader(r: Box<dyn Read>) -> Box<dyn Read> {
    Box::new(BufReader::new(r))
}

fn decode(ci: &mut GeoS2CellID, r: Box<dyn Read>) -> io::Result<()> {
    let mut d = GeoS2Decoder {
        r: geo_s2_as_byte_reader(r),
        err: None,
    };
    decode_cell_id(ci, &mut d);
    match d.err {
        Some(e) => Err(e),
        None => Ok(()),
    }
}

fn decode_cell_id(ci: &mut GeoS2CellID, d: &mut GeoS2Decoder<impl Read>) {
    let value = read_uint64(d);
    *ci = GeoS2CellID(value);
}

fn read_uint64(d: &mut GeoS2Decoder<impl Read>) -> u64 {
    if d.err.is_some() {
        return 0;
    }
    let mut buf = [0u8; 8];
    match d.r.read_exact(&mut buf) {
        Ok(_) => u64::from_le_bytes(buf),
        Err(e) => {
            d.err = Some(e);
            0
        }
    }
}
