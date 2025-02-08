
use std::collections::HashMap;
use std::str::FromStr;
use std::fmt::Display;
use once_cell::sync::Lazy;

struct MoovIoAchFileHeader {
    file_creation_date: String,
}

impl MoovIoAchFileHeader {
    fn file_creation_date_field(&self) -> String {
        let count = self.file_creation_date.chars().count();
        match count {
            0 => format!("{}", Utc::now().formatted),
            6 => self.format_simple_date(&self.file_creation_date),
            _ => {
                let date = DateTime::parse_from_str(&self.file_creation_date, "%+").or(Some(DateTime {
                    formatted: String::new()
                })).map(|d| d.formatted);
                date.unwrap_or_default()
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
            let pad = MOOV_IO_ACH_STRING_ZEROS.get(&(missing as usize)).cloned();
            if let Some(p) = pad {
                format!("{}{}", p, s)
            } else {
                format!("{}{}", "0".repeat(missing as usize), s)
            }
        }
    }
}

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    let mut out = HashMap::with_capacity(94);
    for i in 0..94 {
        out.insert(i, "0".repeat(i));
    }
    out
});

struct Utc;

impl Utc {
    fn now() -> DateTime {
        DateTime {
            formatted: String::new(),
        }
    }
}

struct DateTime {
    formatted: String,
}

impl DateTime {
    fn parse_from_str(_: &str, _: &str) -> Option<Self> {
        Some(Self {
            formatted: String::new(),
        })
    }
}

