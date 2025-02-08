
extern crate lazy_static;

use std::collections::HashMap;
use std::str;
use lazy_static::lazy_static;

lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, Box<String>> = moov_io_ach_populate_map(94, Box::new("0".to_string()));
}

struct MoovIoAchIatEntryDetail {
    rdfi_identification: String,
}

impl MoovIoAchIatEntryDetail {
    fn rdfi_identification_field(&self) -> String {
        moov_io_ach_converters_string_field(&self.rdfi_identification, 8)
    }
}

fn moov_io_ach_converters_string_field(s: &str, max: u32) -> String {
    let ln = s.chars().count() as u32;
    if ln > max {
        return s.chars().take(max as usize).collect();
    }

    let m = (max - ln) as usize;
    if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
        return pad.to_string() + s;
    }

    "0".repeat(m) + s
}

fn moov_io_ach_populate_map(max: usize, zero: Box<String>) -> HashMap<usize, Box<String>> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, Box::new("0".repeat(i)));
    }
    out
}
