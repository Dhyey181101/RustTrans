

use std::collections::HashMap;
use std::str;
use std::time::{SystemTime, UNIX_EPOCH};
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0".to_string()));

fn moov_io_ach_file_header_file_creation_time_field(fh: &Box<MoovIoAchFileHeader>) -> String {
    let count = fh.file_creation_time.chars().count();
    match count {
        0 => {
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            format!("{:0>4}", now % 86400 / 3600 * 100 + now % 3600 / 60)
        }
        4 => fh.moov_io_ach_converters.format_simple_time(&fh.file_creation_time),
        _ => {
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            format!("{:0>4}", now % 86400 / 3600 * 100 + now % 3600 / 60)
        }
    }
}

fn moov_io_ach_converters_format_simple_time(s: &str) -> String {
    if s.is_empty() {
        moov_io_ach_converters_string_field(s, 4)
    } else {
        s.to_string()
    }
}

fn moov_io_ach_converters_string_field(s: &str, max: u32) -> String {
    let ln = s.chars().count() as u32;
    if ln > max {
        s[..max as usize].to_string()
    } else {
        let m = (max - ln) as usize;
        let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap_or(&"".to_string()).to_string();
        pad + s
    }
}

fn moov_io_ach_populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchFileHeader {
    file_creation_time: String,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    fn format_simple_time(&self, s: &str) -> String {
        moov_io_ach_converters_format_simple_time(s)
    }

    fn string_field(&self, s: &str, max: u32) -> String {
        moov_io_ach_converters_string_field(s, max)
    }
}

