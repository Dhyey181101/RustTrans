
use std::io::{self, Read};

pub struct CellID(u64);

struct Decoder<R: Read> {
    r: R,
    err: Option<io::Error>,
}

impl CellID {
    pub fn decode(r: &mut dyn Read) -> Result<Self, io::Error> {
        let mut d = Decoder {
            r,
            err: None,
        };
        let id = read_u64(&mut d)?;
        Ok(Self(id))
    }
}

fn read_u64(d: &mut Decoder<impl Read>) -> Result<u64, io::Error> {
    let mut result = 0;
    d.err = match d.r.read_exact(unsafe { std::slice::from_raw_parts_mut(&mut result as *mut u64 as *mut u8, std::mem::size_of::<u64>()) }) {
        Ok(_) => None,
        Err(e) => Some(e),
    };
    Ok(result)
}

