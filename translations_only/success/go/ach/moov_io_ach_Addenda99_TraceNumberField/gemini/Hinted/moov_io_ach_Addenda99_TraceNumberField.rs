
use std::collections::HashMap;
use std::fmt::Write;

pub struct MoovIoAchAddenda99 {
    pub trace_number: String,
}

impl MoovIoAchAddenda99 {
    pub fn trace_number_field(&self) -> String {
        let mut converters = MoovIoAchConverters {};
        converters.string_field(&self.trace_number, 15)
    }
}

pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(&mut self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let binding = moov_io_ach_string_zeros();
        let pad = binding.get(&m).unwrap();
        format!("{}{}", pad, s)
    }
}

fn moov_io_ach_string_zeros() -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..94 {
        out.insert(i, "0".repeat(i));
    }
    out
}
