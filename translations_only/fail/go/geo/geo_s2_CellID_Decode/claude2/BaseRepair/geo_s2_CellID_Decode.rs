

use std::io::{self, Read};

struct CellID(u64);

struct Decoder<R: Read> {
    r: R,
    err: Option<io::Error>,
}

fn as_byte_reader<R: Read>(r: R) -> R {
    r
}

fn decode(ci: &mut CellID, d: &mut Decoder<impl Read>) {
    *ci = CellID(read_uint64(d));
}

fn read_uint64(d: &mut Decoder<impl Read>) -> u64 {
    let mut x = 0;
    if d.err.is_some() {
        return x;
    }
    let res = read_u64_le(&mut d.r, &mut x);
    d.err = match res {
        Ok(_) => None,
        Err(e) => Some(e),
    };
    x
}

fn read_u64_le(r: &mut impl Read, x: &mut u64) -> Result<(), io::Error> {
    r.read_exact(&mut x.to_le_bytes())
}

