

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct FileHeader {
    pub id: String,
    pub priority_code: String,
    pub immediate_destination: String,
    pub immediate_origin: String,
    pub file_creation_date: String,
    pub file_creation_time: String,
    pub file_id_modifier: String,
    pub record_size: String,
    pub blocking_factor: String,
    pub format_code: String,
    pub immediate_destination_name: String,
    pub immediate_origin_name: String,
    pub reference_code: String,
}

impl FileHeader {
    pub fn file_creation_date_field(&self) -> String {
        match self.file_creation_date.len() {
            0 => SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string(),
            6 => self.format_simple_date(&self.file_creation_date),
            _ => "".to_string(),
        }
    }

    fn format_simple_date(&self, s: &str) -> String {
        if s.is_empty() {
            self.string_field(s, 6)
        } else {
            s.to_string()
        }
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = &STRING_ZEROS[&m];
            format!("{}{}", pad, s)
        }
    }
}

lazy_static! {
    static ref STRING_ZEROS: HashMap<usize, String> =
        (0..94).map(|i| (i, "0".repeat(i))).collect();
}

