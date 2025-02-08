
use std::collections::HashMap;
use std::str::FromStr;
use std::string::ToString;
use lazy_static::lazy_static;

#[derive(Debug)]
pub struct MoovIoAchAddenda99Contested {
    pub original_settlement_date: String,
}

impl MoovIoAchAddenda99Contested {
    pub fn original_settlement_date_field(&self) -> String {
        MoovIoAchConverters {}.string_field(&self.original_settlement_date, 3)
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
        let pad = moov_io_ach_string_zeros(m as usize);
        pad + s
    }
}

fn moov_io_ach_string_zeros(m: usize) -> String {
    let mut out = String::with_capacity(m);
    for _ in 0..m {
        out.push('0');
    }
    out
}

lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, String> =
        moov_io_ach_populate_map(94, "0");
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}
