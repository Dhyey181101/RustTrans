
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub struct MoovIoAchIatBatchHeader {
    pub effective_entry_date: String,
}

impl MoovIoAchIatBatchHeader {
    pub fn effective_entry_date_field(&self) -> String {
        self.effective_entry_date.clone()
    }
}

impl FromStr for MoovIoAchIatBatchHeader {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let effective_entry_date = s.chars().take(6).collect::<String>();
        Ok(MoovIoAchIatBatchHeader {
            effective_entry_date,
        })
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
