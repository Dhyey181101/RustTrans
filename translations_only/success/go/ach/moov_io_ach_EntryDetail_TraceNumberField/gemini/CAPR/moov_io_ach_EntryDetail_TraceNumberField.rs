
use std::collections::HashMap;
use std::fmt::Write;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug, Clone)]
pub struct MoovIoAchEntryDetail {
    pub trace_number: String,
}

impl MoovIoAchEntryDetail {
    pub fn trace_number_field(&self) -> String {
        self.string_field(&self.trace_number, 15)
    }
}

impl MoovIoAchEntryDetail {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s[..max].to_string()
        } else {
            let mut pad = String::with_capacity(max - ln);
            for _ in 0..(max - ln) {
                pad.push('0');
            }
            pad + s
        }
    }
}

pub fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

#[derive(Debug, Clone)]
pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {}
