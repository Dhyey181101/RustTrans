
use std::io::{self, Read, BufReader};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CellID(pub u64);

impl CellID {
    pub fn decode<R: Read>(r: &mut R) -> Result<Self, io::Error> {
        let mut buf = [0u8; 8];
        r.read_exact(&mut buf)?;
        let id = u64::from_le_bytes(buf);
        Ok(CellID(id))
    }
}
