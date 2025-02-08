
#[macro_use]
extern crate lazy_static;
use std::collections::HashMap;
use std::str;

lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, Box<String>> = moov_io_ach_populate_map(94, Box::new("0".to_string()));
}

struct MoovIoAchEntryDetail {
    rdfi_identification: String,
    check_digit: String,
}

impl MoovIoAchEntryDetail {
    fn set_rdfi(&mut self, rdfi: String) -> &mut Self {
        let s = self.string_field(rdfi, 9);
        self.rdfi_identification = self.parse_string_field(&s[..8]);
        self.check_digit = self.parse_string_field(&s[8..9]);
        self
    }

    fn string_field(&self, s: String, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = (max - ln) as usize;
        if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
            return pad.to_string() + &s;
        }

        "0".repeat(m) + &s
    }

    fn parse_string_field(&self, r: &str) -> String {
        r.trim().to_string()
    }
}

fn moov_io_ach_populate_map(max: usize, zero: Box<String>) -> HashMap<usize, Box<String>> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, Box::new("0".repeat(i)));
    }
    out
}
