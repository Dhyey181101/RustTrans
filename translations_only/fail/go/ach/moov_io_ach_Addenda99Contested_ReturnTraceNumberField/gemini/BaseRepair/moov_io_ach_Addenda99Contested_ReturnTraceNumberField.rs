
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug)]
pub struct MoovIoAchAddenda99Contested {
    pub return_trace_number: String,
}

impl MoovIoAchAddenda99Contested {
    pub fn return_trace_number_field(&self) -> &str {
        &self.return_trace_number
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
        out.insert(i, String::from_str("0").unwrap().repeat(i));
    }
    out[&max].clone()
}
