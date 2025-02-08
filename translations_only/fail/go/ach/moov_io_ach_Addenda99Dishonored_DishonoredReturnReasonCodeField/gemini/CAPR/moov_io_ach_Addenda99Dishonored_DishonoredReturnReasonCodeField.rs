
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug)]
pub struct MoovIoAchAddenda99Dishonored {
    pub dishonored_return_reason_code: String,
    pub original_entry_trace_number: String,
    pub original_receiving_dfi_identification: String,
    pub return_trace_number: String,
    pub return_settlement_date: String,
    pub return_reason_code: String,
    pub addenda_information: String,
    pub trace_number: String,
}

impl MoovIoAchAddenda99Dishonored {
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
        let pad = moov_io_ach_string_zeros(m as usize);
        pad + s
    }
}

fn moov_io_ach_string_zeros(max: usize) -> String {
    let mut out = String::with_capacity(max);
    for _ in 0..max {
        out.push('0');
    }
    out
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}
