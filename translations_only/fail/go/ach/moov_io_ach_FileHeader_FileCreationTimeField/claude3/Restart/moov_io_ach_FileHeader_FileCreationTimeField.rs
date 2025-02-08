

use std::collections::HashMap;
use std::str;
use std::time::{SystemTime, UNIX_EPOCH};
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0".to_string()));

fn moov_io_ach_file_header_file_creation_time_field(fh: &Box<MoovIoAchFileHeader>) -> String {
    let count = str::chars(&fh.file_creation_time).count();
    match count {
        0 => {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            format!("{:04}", now % 86400 / 3600 * 100 + now % 3600 / 60)
        }
        4 => fh.moov_io_ach_converters.format_simple_time(&fh.file_creation_time),
        _ => {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            format!("{:04}", now % 86400 / 3600 * 100 + now % 3600 / 60)
        }
    }
}

fn moov_io_ach_converters_format_simple_time(c: &Box<MoovIoAchConverters>, s: &str) -> String {
    if s.is_empty() {
        c.string_field(s, 4)
    } else {
        s.to_string()
    }
}

fn moov_io_ach_converters_string_field(c: &Box<MoovIoAchConverters>, s: &str, max: u32) -> String {
    let len = str::chars(s).count() as u32;
    if len > max {
        s[..max as usize].to_string()
    } else {
        let m = (max - len) as usize;
        if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
            pad.to_string() + s
        } else {
            "0".repeat(m) + s
        }
    }
}

fn moov_io_ach_populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, "0".repeat(i));
    }
    out
}

struct MoovIoAchFileHeader {
    file_creation_time: String,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn format_simple_time(&self, s: &str) -> String {
        moov_io_ach_converters_format_simple_time(&Box::new(MoovIoAchConverters), s)
    }

    fn string_field(&self, s: &str, max: u32) -> String {
        moov_io_ach_converters_string_field(&Box::new(MoovIoAchConverters), s, max)
    }
}

