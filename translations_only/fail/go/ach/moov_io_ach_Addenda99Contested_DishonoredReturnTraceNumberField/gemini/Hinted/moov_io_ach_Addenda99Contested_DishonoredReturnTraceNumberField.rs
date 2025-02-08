
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct MoovIoAchAddenda99Contested {
    pub dishonored_return_trace_number: String,
}

impl MoovIoAchAddenda99Contested {
    pub fn dishonored_return_trace_number_field(&self) -> &str {
        &self.dishonored_return_trace_number
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
        let binding = moov_io_ach_string_zeros();
        let pad = binding.get(&m).unwrap();
        format!("{}{}", pad, s)
    }
}

fn moov_io_ach_string_zeros() -> HashMap<u32, String> {
    let mut out = HashMap::new();
    for i in 0..94 {
        out.insert(i, String::from_str("0").unwrap().repeat(i as usize));
    }
    out
}
