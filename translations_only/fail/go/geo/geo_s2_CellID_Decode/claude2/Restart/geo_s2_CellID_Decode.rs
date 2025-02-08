
use std::io::{self, Read};

struct CellID(u64);

struct Decoder<R: Read> {
    r: R,
    err: Option<io::Error>,
}

fn as_byte_reader<R: Read>(r: R) -> R {
    r
}

fn decode(ci: &mut CellID, d: &mut Decoder<Box<dyn Read>>) {
    *ci = CellID(read_u64(d));
}

fn read_u64(d: &mut Decoder<Box<dyn Read>>) -> u64 {
    let mut result = [0u8; 8];
    if let Err(e) = d.r.read_exact(&mut result) {
        d.err = Some(e);
    }
    u64::from_le_bytes(result)
}
