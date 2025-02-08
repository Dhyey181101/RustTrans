
use std::collections::HashMap;

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
        let binding = moov_io_ach_string_zeros();
        let pad = binding.get(&m).unwrap();
        format!("{}{}", pad, s)
    }
}

fn moov_io_ach_string_zeros() -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..94 {
        out.insert(i, String::from("0").repeat(i));
    }
    out
}
