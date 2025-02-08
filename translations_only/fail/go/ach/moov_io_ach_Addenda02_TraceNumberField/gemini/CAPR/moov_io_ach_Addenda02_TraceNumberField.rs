
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct MoovIoAchAddenda02 {
    pub trace_number: String,
}

impl MoovIoAchAddenda02 {
    pub fn trace_number_field(&self) -> String {
        self.string_field(&self.trace_number, 15)
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = moov_io_ach_string_zeros(m);
            format!("{}{}", pad, s)
        }
    }
}

fn moov_io_ach_string_zeros(max: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, "0".repeat(i));
    }
    out[&max].clone()
}
