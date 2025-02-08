
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug)]
pub struct MoovIoAchAdvEntryDetail {
    pub rdfi_identification: String,
}

impl MoovIoAchAdvEntryDetail {
    pub fn rdfi_identification_field(&self) -> String {
        self.string_field(&self.rdfi_identification, 8)
    }

    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = max - ln;
        let pad = moov_io_ach_string_zeros.get(&(m as i32)).unwrap();
        pad.to_string() + s
    }
}

pub struct MoovIoAchConverters {}

lazy_static! {
    static ref moov_io_ach_string_zeros: HashMap<i32, String> = {
        let mut out = HashMap::new();
        for i in 0..94 {
            out.insert(i, "0".repeat(i as usize));
        }
        out
    };
}
