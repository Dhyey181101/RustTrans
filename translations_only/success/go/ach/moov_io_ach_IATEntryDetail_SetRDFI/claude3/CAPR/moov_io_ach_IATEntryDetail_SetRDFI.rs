

extern crate lazy_static;

use std::collections::HashMap;
use std::str;
use lazy_static::lazy_static;

lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, Box<String>> = {
        let mut map = HashMap::with_capacity(94);
        for i in 0..94 {
            map.insert(i, Box::new("0".repeat(i)));
        }
        map
    };
}

struct MoovIoAchIatEntryDetail {
    rdfi_identification: String,
    check_digit: String,
}

impl MoovIoAchIatEntryDetail {
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

