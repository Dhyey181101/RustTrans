
use std::collections::HashMap;
use std::ops::Deref;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug)]
pub struct MoovIoAchAddenda99Contested {
    pub dishonored_return_settlement_date: String,
}

impl MoovIoAchAddenda99Contested {
    pub fn dishonored_return_settlement_date(&self) -> &str {
        &self.dishonored_return_settlement_date
    }
}

#[derive(Debug)]
pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = max - ln;
        let pad = moov_io_ach_string_zeros(m as usize);
        pad + s
    }
}

fn moov_io_ach_string_zeros(max: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, String::from_utf8(vec![b'0'; i]).unwrap());
    }
    out[&max].clone()
}
