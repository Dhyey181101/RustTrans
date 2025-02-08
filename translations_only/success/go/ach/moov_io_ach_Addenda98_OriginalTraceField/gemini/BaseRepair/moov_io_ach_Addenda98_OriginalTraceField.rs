
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::Write;
use std::str::FromStr;

#[derive(Debug)]
pub struct MoovIoAchAddenda98 {
    pub original_trace: String,
}

impl MoovIoAchAddenda98 {
    pub fn original_trace_field(&self) -> String {
        MoovIoAchConverters {}.string_field(&self.original_trace, 15)
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
        let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap();
        format!("{}{}", pad, s)
    }
}

lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<u32, String> =
        moov_io_ach_populate_map(94, "0");
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<u32, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i as u32, zero.repeat(i));
    }
    out
}
