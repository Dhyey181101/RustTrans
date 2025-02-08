
use std::collections::HashMap;
use std::fmt::Write;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct MoovIoAchAddenda98 {
    pub original_trace: String,
}

impl MoovIoAchAddenda98 {
    pub fn original_trace_field(&self) -> String {
        MoovIoAchConverters {}.string_field(&self.original_trace, 15)
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
        let pad = binding.get(&(m as i32)).unwrap();
        format!("{}{}", pad, s)
    }
}

fn moov_io_ach_string_zeros() -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..94 {
        out.insert(i, String::from_str(&"0".repeat(i as usize)).unwrap());
    }
    out
}
