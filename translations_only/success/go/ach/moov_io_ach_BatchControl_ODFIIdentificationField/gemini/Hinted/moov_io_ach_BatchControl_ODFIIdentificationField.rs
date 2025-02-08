
use lazy_static::lazy_static;
use std::collections::HashMap;

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
    pub fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = max - ln;
        let pad = moov_io_ach_string_zeros.get(&(m as usize)).unwrap();
        let mut out = String::with_capacity(max as usize);
        out.push_str(pad);
        out.push_str(s);
        out
    }
}

lazy_static! {
    static ref moov_io_ach_string_zeros: HashMap<usize, String> = {
        let mut out = HashMap::new();
        for i in 0..94 {
            out.insert(i, "0".repeat(i));
        }
        out
    };
}
