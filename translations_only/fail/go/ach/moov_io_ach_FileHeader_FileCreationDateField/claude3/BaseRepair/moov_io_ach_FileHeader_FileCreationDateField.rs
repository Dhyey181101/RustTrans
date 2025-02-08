
use std::collections::HashMap;
use std::str;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use once_cell::sync::Lazy;
use std::fmt::Debug;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0".to_string()));

fn moov_io_ach_file_header_file_creation_date_field(fh: &Box<MoovIoAchFileHeader>) -> String {
    let count = str::chars(&fh.file_creation_date).count();
    match count {
        0 => {
            let now = SystemTime::now();
            let formatted = format!("{:06}", now.duration_since(UNIX_EPOCH).unwrap().as_secs() % 1_000_000);
            formatted
        }
        6 => fh.converters.format_simple_date(&fh.file_creation_date),
        _ => {
            let t = UNIX_EPOCH + Duration::from_secs(fh.file_creation_date.parse::<u64>().unwrap());
            let formatted = format!("{:06}", t.duration_since(UNIX_EPOCH).unwrap().as_secs() % 1_000_000);
            formatted
        }
    }
}

fn moov_io_ach_converters_format_simple_date(converters: &Box<MoovIoAchConverters>, s: &str) -> String {
    if s.is_empty() {
        converters.string_field(s, 6)
    } else {
        s.to_string()
    }
}

fn moov_io_ach_converters_string_field(converters: &Box<MoovIoAchConverters>, s: &str, max: u32) -> String {
    let ln = str::chars(s).count() as u32;
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
    file_creation_date: String,
    converters: Box<MoovIoAchConverters>,
}

struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    fn format_simple_date(&self, s: &str) -> String {
        moov_io_ach_converters_format_simple_date(&Box::new(MoovIoAchConverters {}), s)
    }

    fn string_field(&self, s: &str, max: u32) -> String {
        moov_io_ach_converters_string_field(&Box::new(MoovIoAchConverters {}), s, max)
    }
}
