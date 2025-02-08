
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Deref;
use std::str::FromStr;
use std::string::ToString;

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
    pub fn string_field<T: Display>(&self, s: T, max: usize) -> String {
        let ln = s.to_string().chars().count();
        if ln > max {
            return s.to_string()[..max].to_string();
        }

        let m = max - ln;
        let pad = moov_io_ach_string_zeros(m);
        format!("{}{}", pad, s)
    }
}

fn moov_io_ach_string_zeros(max: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, "0".repeat(i));
    }
    out[&max].clone()
}
