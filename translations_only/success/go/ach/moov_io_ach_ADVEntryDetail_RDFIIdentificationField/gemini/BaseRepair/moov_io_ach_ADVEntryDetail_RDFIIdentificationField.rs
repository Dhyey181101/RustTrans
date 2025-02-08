
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct MoovIoAchAdvEntryDetail {
    pub rdfi_identification: String,
}

impl MoovIoAchAdvEntryDetail {
    pub fn rdfi_identification_field(&self) -> String {
        MoovIoAchConverters::string_field(&self.rdfi_identification, 8)
    }
}

pub struct MoovIoAchConverters;

impl MoovIoAchConverters {
    pub fn string_field(s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = max - ln;
        let pad = MOOV_IO_ACH_STRING_ZEROS.get(&(m as usize)).unwrap();
        pad.to_string() + s
    }
}

lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, String> =
        moov_io_ach_populate_map(94, "0");
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}
