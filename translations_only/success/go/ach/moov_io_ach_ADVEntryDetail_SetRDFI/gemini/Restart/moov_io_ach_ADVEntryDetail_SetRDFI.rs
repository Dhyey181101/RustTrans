
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct MoovIoAchAdvEntryDetail {
    pub rdfi_identification: String,
    pub check_digit: String,
}

impl MoovIoAchAdvEntryDetail {
    pub fn set_rdfi(&mut self, rdfi: &str) -> &mut Self {
        let s = rdfi.chars().take(8).collect::<String>();
        self.rdfi_identification = s;
        self.check_digit = rdfi.chars().nth(8).unwrap().to_string();
        self
    }
}

pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let pad = &MOOV_IO_ACH_STRING_ZEROS[&m];
        format!("{}{}", pad, s)
    }

    pub fn parse_string_field(&self, r: &str) -> String {
        r.trim().to_string()
    }
}

lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, String> = {
        moov_io_ach_populate_map(94, "0")
    };
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}
