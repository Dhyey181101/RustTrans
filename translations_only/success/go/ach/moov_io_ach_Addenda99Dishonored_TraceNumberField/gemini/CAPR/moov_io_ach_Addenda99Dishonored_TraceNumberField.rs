
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct MoovIoAchAddenda99Dishonored {
    pub trace_number: String,
}

impl MoovIoAchAddenda99Dishonored {
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

fn moov_io_ach_string_zeros(m: usize) -> String {
    let mut out = String::with_capacity(m);
    for _ in 0..m {
        out.push('0');
    }
    out
}

fn moov_io_ach_populate_map(max: usize, zero: char) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, String::from_str(&zero.to_string().repeat(i)).unwrap());
    }
    out
}
