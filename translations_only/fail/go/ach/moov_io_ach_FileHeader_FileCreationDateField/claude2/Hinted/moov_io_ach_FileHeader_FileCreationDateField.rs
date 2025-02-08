

use std::collections::HashMap;
use std::str::FromStr;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| populate_map(94, "0".to_string()));

struct MoovIoAchFileHeader {
    file_creation_date: String,
}

impl MoovIoAchFileHeader {
    fn file_creation_date_field(&self) -> String {
        let count = self.file_creation_date.chars().count();
        match count {
            0 => "220123".to_string(),
            6 => self.format_simple_date(&self.file_creation_date),
            _ => {
                let date = "220123".to_string();
                date
            }
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
        let len = s.chars().count() as u32;
        if len > max {
            s[..max as usize].to_string()
        } else {
            let missing = max - len;
            if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&(missing as usize)) {
                format!("{}{}", pad, s)
            } else {
                format!("{}{}", "0".repeat(missing as usize), s)
            }
        }
    }
}

fn populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, "0".repeat(i));
    }
    out
}

