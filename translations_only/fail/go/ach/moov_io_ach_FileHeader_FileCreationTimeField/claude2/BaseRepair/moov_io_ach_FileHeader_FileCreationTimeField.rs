
use std::time::{SystemTime, UNIX_EPOCH};
use std::string::ToString;

struct FileHeader {
    file_creation_time: String,
}

impl FileHeader {
    fn file_creation_time_field(&self) -> String {
        match self.file_creation_time.len() {
            0 => SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string(),
            4 => self.format_simple_time(&self.file_creation_time),
            _ => self.file_creation_time.clone()
        }
    }

    fn format_simple_time(&self, s: &str) -> String {
        if s.is_empty() {
            return Converters::string_field(s, 4);
        }
        s.to_string()
    }
}

struct Converters;

impl Converters {
    fn string_field(s: &str, max: u32) -> String {
        let len = s.len() as u32;
        if len > max {
            return s[..max as usize].to_string();
        }
        let mut pad = String::new();
        for _ in 0..(max - len) {
            pad.push('0');
        }
        pad + s
    }
}

