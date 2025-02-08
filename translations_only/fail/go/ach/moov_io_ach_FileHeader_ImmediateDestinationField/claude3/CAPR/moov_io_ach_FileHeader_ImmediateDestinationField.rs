

extern crate lazy_static;

use std::collections::HashMap;
use std::str;
use lazy_static::lazy_static;

lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, String> = moov_io_ach_populate_map(94, "0".to_string());
}

fn moov_io_ach_file_header_immediate_destination_field(fh: &Box<MoovIoAchFileHeader>) -> String {
    if fh.immediate_destination.is_empty() {
        return " ".repeat(10);
    }
    let immediate_destination = fh.immediate_destination.trim();
    if let Some(validate_opts) = fh.validate_opts.as_ref() {
        if validate_opts.bypass_destination_validation && immediate_destination.len() == 10 {
            return immediate_destination.to_string();
        }
    }
    format!(" {}", moov_io_ach_string_field(&immediate_destination, 9))
}

fn moov_io_ach_string_field(s: &str, max: u32) -> String {
    let ln = s.chars().count() as u32;
    if ln > max {
        return s[..max as usize].to_string();
    }

    let m = (max - ln) as usize;
    if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
        return format!("{}{}", pad, s);
    }

    format!("{}{}", "0".repeat(m), s)
}

fn moov_io_ach_populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchFileHeader {
    immediate_destination: String,
    validate_opts: Box<Option<MoovIoAchValidateOpts>>,
    converters: MoovIoAchConverters,
}

struct MoovIoAchValidateOpts {
    bypass_destination_validation: bool,
}

struct MoovIoAchConverters {}

