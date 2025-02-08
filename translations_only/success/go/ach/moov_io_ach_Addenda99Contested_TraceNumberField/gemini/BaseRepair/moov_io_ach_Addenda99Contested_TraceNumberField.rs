
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug)]
pub struct MoovIoAchAddenda99Contested {
    pub trace_number: String,
}

impl MoovIoAchAddenda99Contested {
    pub fn trace_number_field(&self) -> String {
        self.string_field(&self.trace_number, 15)
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let binding = moov_io_ach_string_zeros();
        let pad = binding.get(&m).unwrap();
        pad.to_string() + s
    }
}

fn moov_io_ach_string_zeros() -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..94 {
        out.insert(i, String::from_str("0").unwrap().repeat(i));
    }
    out
}
