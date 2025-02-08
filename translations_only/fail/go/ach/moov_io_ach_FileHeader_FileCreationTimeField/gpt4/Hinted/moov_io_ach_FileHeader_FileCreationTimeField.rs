
use std::collections::HashMap;

lazy_static::lazy_static! {
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
    file_creation_time: String,
}

struct MoovIoAchConverters;

impl MoovIoAchFileHeader {
    fn file_creation_time_field(&self) -> String {
        self.format_simple_time(&self.file_creation_time)
    }

    fn format_simple_time(&self, s: &str) -> String {
        if s.is_empty() {
            MoovIoAchConverters::string_field(s, 4)
        } else {
            s.to_string()
        }
    }
}

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
