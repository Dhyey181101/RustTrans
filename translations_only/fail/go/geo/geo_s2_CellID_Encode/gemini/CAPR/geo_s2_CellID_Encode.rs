
use std::io::{Error, Write};

pub fn encode(ci: u64, w: &mut dyn Write) -> Result<(), Error> {
    write_u64(w, ci)
}

fn write_u64(w: &mut dyn Write, x: u64) -> Result<(), Error> {
    w.write_all(&x.to_le_bytes())
}
