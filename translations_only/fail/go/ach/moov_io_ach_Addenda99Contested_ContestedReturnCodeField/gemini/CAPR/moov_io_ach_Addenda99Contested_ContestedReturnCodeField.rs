

use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug)]
pub struct MoovIoAchAddenda99Contested {
    pub contested_return_code: String,
}

impl MoovIoAchAddenda99Contested {
    pub fn contested_return_code_field(&self) -> String {
        MoovIoAchConverters {}.string_field(&self.contested_return_code, 3)
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
        let pad = moov_io_ach_string_zeros(m);
        pad + s
    }
}

fn moov_io_ach_string_zeros(max: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, "0".repeat(i));
    }
    out[&max].clone()
}

