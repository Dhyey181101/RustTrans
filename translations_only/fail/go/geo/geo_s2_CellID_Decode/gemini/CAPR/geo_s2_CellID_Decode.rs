
use std::io::{self, Read};

fn decode_cell_id(r: &mut dyn Read) -> Result<u64, io::Error> {
    let mut buf = [0u8; 8];
    r.read_exact(&mut buf)?;
    Ok(u64::from_le_bytes(buf))
}
