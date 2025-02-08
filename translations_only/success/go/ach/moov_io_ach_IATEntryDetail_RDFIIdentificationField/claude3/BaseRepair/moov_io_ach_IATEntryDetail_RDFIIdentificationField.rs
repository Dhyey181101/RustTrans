
#[macro_use]
extern crate lazy_static;
use std::collections::HashMap;
use std::str;

lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, String> = moov_io_ach_populate_map(94, "0".to_string());
}

fn moov_io_ach_iat_entry_detail_rdfi_identification_field(iat_ed: &Box<moov_io_ach_IATEntryDetail>) -> String {
    moov_io_ach_string_field(&iat_ed.rdfi_identification, 8)
}

fn moov_io_ach_string_field(s: &str, max: u32) -> String {
    let ln = s.chars().count() as u32;
    if ln > max {
        return s[..max as usize].to_string();
    }

    let m = (max - ln) as usize;
    if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
        return pad.to_string() + s;
    }

    "0".repeat(m) + s
}

fn moov_io_ach_populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, "0".repeat(i));
    }
    out
}

struct moov_io_ach_IATEntryDetail {
    rdfi_identification: String,
    moov_io_ach_converters: Box<moov_io_ach_converters>,
}

struct moov_io_ach_converters {}
