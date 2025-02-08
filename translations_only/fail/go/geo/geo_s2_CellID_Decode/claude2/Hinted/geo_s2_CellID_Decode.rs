
use std::io;

struct CellID(u64);

struct Decoder {
    r: Box<dyn io::Read>,
    err: Option<io::Error>,
}

fn as_byte_reader(r: Box<dyn io::Read>) -> Box<dyn io::Read> {
    r
}

fn decode(ci: &mut CellID, d: &mut Decoder) {
    *ci = CellID(read_u64(d));
}

fn read_u64(d: &mut Decoder) -> u64 {
    let mut x = 0;
    if let Some(ref mut err) = d.err {
        return x;
    }
    let res = read_u64_inner(d.r.as_mut(), &mut x);
    d.err = match res {
        Ok(_) => None,
        Err(e) => Some(e),
    };
    x
}

fn read_u64_inner(r: &mut dyn io::Read, x: &mut u64) -> Result<(), io::Error> {
    r.read_exact(&mut x.to_ne_bytes())
}

