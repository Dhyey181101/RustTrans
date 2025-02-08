
use std::io::{BufReader, Read, Seek, SeekFrom};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CellID(pub u64);

impl CellID {
    pub fn decode<R: Read + Seek>(r: &mut R) -> Result<Self, std::io::Error> {
        let mut buf = [0u8; 8];
        r.read_exact(&mut buf)?;
        Ok(CellID(u64::from_le_bytes(buf)))
    }
}
