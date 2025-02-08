
use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, String> = moov_io_ach_populate_map(94, "0");
}

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
        "".to_string()
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
    fn string_field(s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s.chars().take(max).collect()
        } else {
            let m = max - ln;
            if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
                format!("{}{}", pad, s)
            } else {
                "0".repeat(m) + s
            }
        }
    }
}

fn main() {}
