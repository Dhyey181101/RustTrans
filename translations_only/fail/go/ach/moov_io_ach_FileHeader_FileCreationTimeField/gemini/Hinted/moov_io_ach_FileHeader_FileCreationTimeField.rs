
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct FileHeader {
    pub file_creation_time: String,
}

impl FileHeader {
    pub fn file_creation_time_field(&self) -> String {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let time_str = time.to_string();
        if time_str.len() == 4 {
            return time_str;
        }
        time_str[..4].to_string()
    }
}

pub struct Converters {}

impl Converters {
    pub fn format_simple_time(&self, s: &str) -> String {
        if s.is_empty() {
            return String::from("0000");
        }
        s.to_string()
    }

    pub fn string_field(&self, s: &str, max: usize) -> String {
        let len = s.chars().count();
        if len > max {
            return s[..max].to_string();
        }

        let mut pad = String::new();
        for _ in 0..(max - len) {
            pad.push('0');
        }
        pad + s
    }
}

pub fn populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}
