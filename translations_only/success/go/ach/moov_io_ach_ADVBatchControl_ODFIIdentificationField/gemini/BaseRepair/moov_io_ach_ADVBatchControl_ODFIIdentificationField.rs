
use std::collections::HashMap;
use std::str::FromStr;
use std::string::ToString;

pub struct MoovIoAchAdvBatchControl {
    pub odfi_identification: String,
}

pub struct MoovIoAchConverters {}

impl MoovIoAchAdvBatchControl {
    pub fn odfi_identification_field(&self) -> String {
        MoovIoAchConverters {}.string_field(&self.odfi_identification, 8)
    }
}

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

fn moov_io_ach_string_zeros(m: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..m {
        out.insert(i, "0".repeat(i));
    }
    out[&m].to_string()
}
