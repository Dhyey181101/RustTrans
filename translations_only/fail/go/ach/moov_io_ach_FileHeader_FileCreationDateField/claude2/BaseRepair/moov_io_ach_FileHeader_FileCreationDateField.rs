
use std::collections::HashMap;
use std::str::FromStr;
use std::string::String;

struct MoovIoAchFileHeader {
    file_creation_date: String,
}

impl MoovIoAchFileHeader {
    fn file_creation_date_field(&self) -> String {
        let count = self.file_creation_date.len();
        match count {
            0 => String::from("000000"),
            6 => self.format_simple_date(&self.file_creation_date),
            _ => String::from("000000"),
        }
    }

    fn format_simple_date(&self, s: &str) -> String {
        if s.is_empty() {
            MoovIoAchConverters::string_field(s, 6)
        } else {
            s.to_string()
        }
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(s: &str, max: u32) -> String {
        let len = s.len() as u32;
        if len > max {
            s[..max as usize].to_string()
        } else {
            let missing = max - len;
            let pad = "0".repeat(missing as usize);
            format!("{}{}", pad, s)
        }
    }
}

