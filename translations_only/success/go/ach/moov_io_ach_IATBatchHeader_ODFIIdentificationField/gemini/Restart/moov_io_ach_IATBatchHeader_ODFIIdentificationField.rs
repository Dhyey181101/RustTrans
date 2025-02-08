
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::str::FromStr;
use std::string::ToString;
use std::usize;

#[derive(Debug)]
pub struct MoovIoAchIatBatchHeader {
    pub odfi_identification: String,
}

impl MoovIoAchIatBatchHeader {
    pub fn odfi_identification_field(&self) -> String {
        self.string_field(&self.odfi_identification, 8)
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let binding = "".repeat(m);
        let pad = moov_io_ach_string_zeros.get(&m).unwrap_or(&binding);
        format!("{}{}", pad, s)
    }
}

lazy_static! {
    static ref moov_io_ach_string_zeros: HashMap<usize, String> =
        (0..94).map(|i| (i, "0".repeat(i))).collect();
}
