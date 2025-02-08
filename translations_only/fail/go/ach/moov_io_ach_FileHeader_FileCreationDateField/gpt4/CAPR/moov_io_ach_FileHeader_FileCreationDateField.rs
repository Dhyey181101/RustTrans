
use std::collections::HashMap;
use std::sync::Mutex;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchFileHeader {
    file_creation_date: String,
}

impl MoovIoAchFileHeader {
    fn file_creation_date_field(&self) -> String {
        match self.file_creation_date.chars().count() {
            0 => "000000".to_string(), // Placeholder for current date in YYMMDD format
            6 => self.format_simple_date(&self.file_creation_date),
            _ => "".to_string(), // Placeholder for error or unsupported format handling
        }
    }

    fn format_simple_date(&self, s: &str) -> String {
        if s.is_empty() {
            string_field(s, 6)
        } else {
            s.to_string()
        }
    }
}

fn string_field(s: &str, max: usize) -> String {
    let ln = s.chars().count();
    if ln > max {
        s.chars().take(max).collect()
    } else {
        let m = max - ln;
        let pad = MOOV_IO_ACH_STRING_ZEROS.lock().unwrap().get(&m).unwrap().clone();
        format!("{}{}", pad, s)
    }
}

lazy_static::lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: Mutex<HashMap<usize, String>> = Mutex::new(moov_io_ach_populate_map(94, "0"));
}

fn main() {}
