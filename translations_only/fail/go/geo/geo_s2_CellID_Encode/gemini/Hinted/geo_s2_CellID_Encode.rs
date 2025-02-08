
use std::io::{Error, Write};

pub fn encode_cell_id(cell_id: u64, w: &mut dyn Write) -> Result<(), Error> {
    write_u64(cell_id, w)
}

fn write_u64(x: u64, w: &mut dyn Write) -> Result<(), Error> {
    w.write_all(&x.to_le_bytes())
}
