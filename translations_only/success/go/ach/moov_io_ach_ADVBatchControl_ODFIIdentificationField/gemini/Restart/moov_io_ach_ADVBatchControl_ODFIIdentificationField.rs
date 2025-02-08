

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug)]
pub struct MoovIoAchAdvBatchControl {
    pub odfi_identification: String,
}

impl MoovIoAchAdvBatchControl {
    pub fn odfi_identification_field(&self) -> String {
        self.string_field(&self.odfi_identification, 8)
    }
}

impl MoovIoAchAdvBatchControl {
    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = max - ln;
        let pad = moov_io_ach_string_zeros.get(&(m as i32)).unwrap();
        format!("{}{}", pad, s)
    }
}

lazy_static! {
    static ref moov_io_ach_string_zeros: HashMap<i32, String> =
        moov_io_ach_populate_map(94, "0");
}

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

