
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
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
        let effective_entry_date = s.get(0..6).unwrap_or("").to_string();

        Ok(MoovIoAchIatBatchHeader {
            effective_entry_date,
        })
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
        let pad = moov_io_ach_string_zeros.get(&(m as i32)).unwrap();
        format!("{}{}", pad, s)
    }
}

lazy_static! {
    static ref moov_io_ach_string_zeros: HashMap<i32, String> = {
        let mut out = HashMap::new();
        for i in 0..94 {
            out.insert(i, "0".repeat(i as usize));
        }
        out
    };
}
