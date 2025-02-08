
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct MoovIoAchAddenda98 {
    pub original_dfi: String,
    pub corrected_data: String,
}

impl MoovIoAchAddenda98 {
    pub fn original_dfi_field(&self) -> &str {
        &self.original_dfi
    }
}

#[derive(Debug, Clone)]
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
        out.insert(i, String::from_str("0").unwrap().repeat(i));
    }
    out
}
