

use std::collections::HashMap;
use std::str;
use std::time::{SystemTime, UNIX_EPOCH};
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, Box<str>>> =
    Lazy::new(|| moov_io_ach_populate_map(94, Box::from("0")));

fn moov_io_ach_file_header_file_creation_time_field(fh: &Box<MoovIoAchFileHeader>) -> String {
    let rune_count = str::chars(fh.file_creation_time.as_str()).count();
    match rune_count {
        0 => {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            format!("{:0>4}", now % 86400 / 3600)
        }
        4 => moov_io_ach_converters_format_simple_time(&fh.file_creation_time),
        _ => {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            format!("{:0>4}", now % 86400 / 3600)
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
    let ln = str::chars(s).count() as u32;
    if ln > max {
        s[..max as usize].to_string()
    } else {
        let m = (max - ln) as usize;
        let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap_or(&Box::from("")).to_string();
        pad + s
    }
}

fn moov_io_ach_populate_map(max: usize, zero: Box<str>) -> HashMap<usize, Box<str>> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, Box::from(zero.repeat(i)));
    }
    out
}

struct MoovIoAchFileHeader {
    file_creation_time: String,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

struct MoovIoAchConverters {}

