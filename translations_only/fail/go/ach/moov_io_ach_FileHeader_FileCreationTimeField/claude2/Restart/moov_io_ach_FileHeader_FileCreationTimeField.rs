
use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt;

struct FileHeader {
    file_creation_time: String,
}

impl FileHeader {
    fn file_creation_time_field(&self) -> String {
        match self.file_creation_time.len() {
            0 => Utc::now().format("%H%M").to_string(),
            4 => self.format_simple_time(&self.file_creation_time),
            _ => {
                let dt = DateTime::parse_from_rfc3339(&self.file_creation_time)
                    .unwrap_or_else(|_| Utc::now());
                dt.format("%H%M").to_string()
            }
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

struct Utc;

impl Utc {
    fn now() -> DateTime {
        unimplemented!()
    }
}

struct DateTime {

}

impl DateTime {
    fn format(&self, _: &str) -> String {
        String::new() 
    }

    fn parse_from_rfc3339(_: &str) -> Result<DateTime, ()> {
        unimplemented!()
    }
}

