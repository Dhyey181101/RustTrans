
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use std::string::ToString;
use std::usize;

#[derive(Debug)]
pub struct MoovIoAchIatBatchHeader {
    pub effective_entry_date: String,
}

impl MoovIoAchIatBatchHeader {
    pub fn effective_entry_date_field(&self) -> String {
        self.effective_entry_date.clone()
    }
}

impl ToString for MoovIoAchIatBatchHeader {
    fn to_string(&self) -> String {
        format!(
            "EffectiveEntryDate: {}, ",
            self.effective_entry_date_field()
        )
    }
}

#[derive(Debug)]
pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
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
