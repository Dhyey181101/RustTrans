
#[macro_use]
extern crate lazy_static;
use std::collections::HashMap;
use std::str;

lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, Box<String>> = moov_io_ach_populate_map(94, Box::new("0".to_string()));
}

struct MoovIoAchIatEntryDetail {
    rdfi_identification: Box<String>,
    check_digit: Box<String>,
}

impl MoovIoAchIatEntryDetail {
    fn set_rdfi(&mut self, rdfi: Box<String>) -> &mut Self {
        let s = string_field(rdfi, 9);
        self.rdfi_identification = Box::new(parse_string_field(s[..8].to_string()));
        self.check_digit = Box::new(parse_string_field(s[8..9].to_string()));
        self
    }
}

fn string_field(s: Box<String>, max: u32) -> Box<String> {
    let ln = s.chars().count() as u32;
    if ln > max {
        return Box::new(s[..max as usize].to_string());
    }

    let m = (max - ln) as usize;
    let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap_or(&Box::new("0".repeat(m))).to_string();
    Box::new(pad + &s)
}

fn parse_string_field(r: String) -> String {
    r.trim().to_string()
}

fn moov_io_ach_populate_map(max: usize, zero: Box<String>) -> HashMap<usize, Box<String>> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, Box::new("0".repeat(i)));
    }
    out
}
