
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Deref;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug, Clone)]
pub struct MoovIoAchAddenda99Contested {
    pub dishonored_return_reason_code: String,
}

impl MoovIoAchAddenda99Contested {
    pub fn dishonored_return_reason_code_field(&self) -> &str {
        &self.dishonored_return_reason_code
    }
}

#[derive(Debug, Clone)]
pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = max - ln;
        let binding = moov_io_ach_string_zeros();
        let pad = binding.get(&(m as i32)).unwrap();
        format!("{}{}", pad, s)
    }
}

fn moov_io_ach_string_zeros() -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..94 {
        out.insert(i, "0".repeat(i as usize));
    }
    out
}
