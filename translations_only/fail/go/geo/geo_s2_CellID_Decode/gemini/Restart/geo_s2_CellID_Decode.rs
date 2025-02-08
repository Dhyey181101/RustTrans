
use std::io::{BufRead, Read, Error};

pub fn decode(ci: &mut u64, r: &mut dyn Read) -> Result<(), Error> {
    let mut buf = [0; 8];
    r.read_exact(&mut buf)?;
    *ci = u64::from_le_bytes(buf);
    Ok(())
}
