

use std::collections::HashMap;
use std::str;
use std::time::{SystemTime, UNIX_EPOCH};
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, Box<str>>> = Lazy::new(|| moov_io_ach_populate_map(94, Box::from("0")));

struct MoovIoAchFileHeader {
    file_creation_date: String,
    converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchFileHeader {
    fn file_creation_date_field(&self) -> String {
        let rune_count = str::chars(&self.file_creation_date).count();
        match rune_count {
            0 => SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string(),
            6 => self.converters.format_simple_date(self.file_creation_date.clone()),
            _ => {
                let t = SystemTime::now();
                let formatted = format!("{:06}", t.duration_since(UNIX_EPOCH).unwrap().as_secs());
                formatted
            }
        }
    }
}

struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    fn format_simple_date(&self, s: String) -> String {
        if s.is_empty() {
            self.string_field(s, 6)
        } else {
            s
        }
    }

    fn string_field(&self, s: String, max: u32) -> String {
        let ln = str::chars(&s).count() as u32;
        if ln > max {
            s.chars().take(max as usize).collect()
        } else {
            let m = (max - ln) as usize;
            let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).cloned().unwrap_or_else(|| Box::from(""));
            pad.to_string() + &s
        }
    }
}

fn moov_io_ach_populate_map(max: usize, zero: Box<str>) -> HashMap<usize, Box<str>> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, Box::from(zero.repeat(i)));
    }
    out
}

