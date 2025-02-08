
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct MoovIoAchFileHeader {
    pub file_creation_date: String,
}

impl MoovIoAchFileHeader {
    pub fn file_creation_date_field(&self) -> String {
        if self.file_creation_date.len() == 0 {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string()
        } else if self.file_creation_date.len() == 6 {
            self.file_creation_date.clone()
        } else {
            "".to_string()
        }
    }
}

pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn format_simple_date(&self, s: &str) -> String {
        if s.is_empty() {
            String::from("000000")
        } else {
            s.to_string()
        }
    }

    pub fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s[..max].to_string()
        } else {
            let mut pad = String::new();
            for _ in 0..(max - ln) {
                pad.push('0');
            }
            pad + s
        }
    }
}

pub fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}
