
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Deref;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoovIoAchAddenda99Contested {
    pub original_entry_trace_number: String,
    pub date_original_entry_returned: String,
    pub original_receiving_dfi_identification: String,
    pub original_settlement_date: String,
    pub return_trace_number: String,
    pub return_settlement_date: String,
    pub return_reason_code: String,
    pub dishonored_return_trace_number: String,
    pub dishonored_return_settlement_date: String,
    pub dishonored_return_reason_code: String,
    pub trace_number: String,
}

impl MoovIoAchAddenda99Contested {
    pub fn original_entry_trace_number_field(&self) -> &str {
        &self.original_entry_trace_number
    }
}

pub struct MoovIoAchConverters {}

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
        out.insert(i, "0".repeat(i));
    }
    out[&max].clone()
}
