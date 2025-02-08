

use std::collections::HashMap;
use std::str;
use std::time::{SystemTime, UNIX_EPOCH};
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0".to_string()));

fn moov_io_ach_file_header_file_creation_date_field(fh: &Box<MoovIoAchFileHeader>) -> String {
    let count = str::chars(&fh.file_creation_date).count();
    match count {
        0 => {
            let start = SystemTime::now();
            let since_the_epoch = start
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards");
            format!("{:06}", since_the_epoch.as_secs() % 1_000_000)
        }
        6 => format_simple_date(&fh.moov_io_ach_converters, &fh.file_creation_date),
        _ => {
            if let Ok(duration) = fh.file_creation_date.parse::<u64>() {
                let t = UNIX_EPOCH + std::time::Duration::from_secs(duration);
                let since_the_epoch = t
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards");
                format!("{:06}", since_the_epoch.as_secs() % 1_000_000)
            } else {
                String::new()
            }
        }
    }
}

fn format_simple_date(c: &Box<MoovIoAchConverters>, s: &str) -> String {
    if s.is_empty() {
        (c.string_field)(c.as_ref(), s, 6)
    } else {
        s.to_string()
    }
}

fn string_field(c: &MoovIoAchConverters, s: &str, max: u32) -> String {
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
        out.insert(i, "0".repeat(i));
    }
    out
}

struct MoovIoAchFileHeader {
    file_creation_date: String,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

struct MoovIoAchConverters {
    format_simple_date: Box<dyn Fn(&Box<MoovIoAchConverters>, &str) -> String>,
    string_field: Box<dyn Fn(&MoovIoAchConverters, &str, u32) -> String>,
}

