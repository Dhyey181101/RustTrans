
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug)]
pub struct MoovIoAchEntryDetail {
    pub trace_number: String,
}

impl MoovIoAchEntryDetail {
    pub fn trace_number_field(&self) -> String {
        let s = &self.trace_number;
        let ln = s.chars().count();
        if ln > 15 {
            s[..15].to_string()
        } else {
            let m = 15 - ln;
            let pad = moov_io_ach_string_zeros.get(&m).unwrap();
            format!("{}{}", pad, s)
        }
    }
}

lazy_static! {
    static ref moov_io_ach_string_zeros: HashMap<usize, String> = moov_io_ach_populate_map(94, "0");
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    (0..max).map(|i| (i, zero.repeat(i))).collect()
}
