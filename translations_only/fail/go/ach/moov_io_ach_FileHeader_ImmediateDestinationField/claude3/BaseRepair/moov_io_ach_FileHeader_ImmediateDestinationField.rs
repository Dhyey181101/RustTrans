
#[macro_use]
extern crate lazy_static;
use std::collections::HashMap;
use std::str;

lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, String> = moov_io_ach_populate_map(94, "0".to_string());
}

impl moov_io_ach_FileHeader {
    fn immediate_destination_field(&self) -> String {
        if self.immediate_destination.is_empty() {
            return " ".repeat(10);
        }
        let immediate_destination = self.immediate_destination.trim();
        if self.validate_opts.is_some()
            && self.validate_opts.as_ref().unwrap().bypass_destination_validation
            && immediate_destination.len() == 10
        {
            return immediate_destination.to_string();
        }
        let mut result = " ".to_string();
        result.push_str(&self.converters.string_field(immediate_destination, 9));
        result
    }
}

impl moov_io_ach_converters {
    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = (max - ln) as usize;
        if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
            return pad.to_string() + s;
        }
        "0".repeat(m) + s
    }
}

fn moov_io_ach_populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct moov_io_ach_FileHeader {
    immediate_destination: String,
    validate_opts: Option<Box<moov_io_ach_ValidateOpts>>,
    converters: moov_io_ach_converters,
}

struct moov_io_ach_ValidateOpts {
    bypass_destination_validation: bool,
}

struct moov_io_ach_converters;
