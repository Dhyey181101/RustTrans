
use std::collections::HashMap;
use std::str;
use std::time::{SystemTime, UNIX_EPOCH};
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0".to_string()));

fn moov_io_ach_file_header_file_creation_date_field(fh: &Box<MoovIoAchFileHeader>) -> String {
    let count = str::chars(&fh.file_creation_date).count();
    match count {
        0 => SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string(),
        6 => fh.moov_io_ach_converters.format_simple_date(&fh.file_creation_date),
        _ => {
            let t = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string();
            t[t.len() - 6..].to_string()
        }
    }
}

fn moov_io_ach_converters_format_simple_date(converters: &Box<MoovIoAchConverters>, s: &str) -> String {
    converters.moov_io_ach_converters_string_field(s, 6)
}

fn moov_io_ach_converters_string_field(converters: &Box<MoovIoAchConverters>, s: &str, max: u32) -> String {
    let ln = str::chars(s).count() as u32;
    if ln > max {
        s[..max as usize].to_string()
    } else {
        let m = (max - ln) as usize;
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
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchFileHeader {
    file_creation_date: String,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    fn format_simple_date(&self, s: &str) -> String {
        self.moov_io_ach_converters_string_field(s, 6)
    }

    fn moov_io_ach_converters_string_field(&self, s: &str, max: u32) -> String {
        let ln = str::chars(s).count() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let m = (max - ln) as usize;
            if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
                pad.to_string() + s
            } else {
                "0".repeat(m) + s
            }
        }
    }
}
