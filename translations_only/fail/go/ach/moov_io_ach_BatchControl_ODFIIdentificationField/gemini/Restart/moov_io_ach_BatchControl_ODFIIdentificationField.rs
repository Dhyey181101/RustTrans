

use std::collections::HashMap;
use std::ops::Deref;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug, Clone)]
pub struct MoovIoAchBatchControl {
    pub odfi_identification: String,
}

impl MoovIoAchBatchControl {
    pub fn odfi_identification_field(&self) -> &str {
        self.odfi_identification.as_str()
    }
}

#[derive(Debug, Clone)]
pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = max - ln;
        let pad = moov_io_ach_string_zeros(m as usize);
        pad + s
    }
}

fn moov_io_ach_string_zeros(max: usize) -> String {
    let mut out = String::new();
    for i in 0..max {
        out.push_str(&"0".repeat(i));
    }
    out
}

