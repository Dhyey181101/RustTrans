
use std::collections::HashMap;
use std::str::FromStr;
use std::string::ToString;
use std::usize;

pub struct MoovIoAchBatchControl {
    pub odfi_identification: String,
}

pub struct MoovIoAchConverters {}

impl MoovIoAchBatchControl {
    pub fn odfi_identification_field(&self) -> String {
        MoovIoAchConverters {}.string_field(&self.odfi_identification, 8)
    }
}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let pad = moov_io_ach_string_zeros(m);
        pad + s
    }
}

fn moov_io_ach_string_zeros(max: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, String::from_str("0").unwrap().repeat(i));
    }
    out[&max].clone()
}
