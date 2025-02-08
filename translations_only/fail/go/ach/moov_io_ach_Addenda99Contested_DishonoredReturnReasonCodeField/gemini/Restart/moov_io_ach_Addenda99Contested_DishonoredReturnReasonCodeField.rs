
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug)]
pub struct MoovIoAchAddenda99Contested {
    pub dishonored_return_reason_code: String,
}

impl MoovIoAchAddenda99Contested {
    pub fn dishonored_return_reason_code_field(&self) -> &str {
        &self.dishonored_return_reason_code
    }
}

pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = max - ln;
        let pad = MOOV_IO_ACH_STRING_ZEROS.get(&(m as i32)).unwrap();
        format!("{}{}", pad, s)
    }
}

lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<i32, String> =
        moov_io_ach_populate_map(94, "0");
}

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}
