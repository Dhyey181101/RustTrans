
use std::collections::HashMap;
use std::str::FromStr;
use std::string::ToString;
use lazy_static::lazy_static;

#[derive(Debug)]
pub struct MoovIoAchFileHeader {
    pub immediate_origin: String,
    pub file_creation_date: String,
    pub file_creation_time: String,
    pub record_size: String,
    pub blocking_factor: String,
    pub format_code: String,
    pub immediate_destination_name: String,
    pub immediate_origin_name: String,
    pub reference_code: String,
    pub validate_opts: Option<MoovIoAchValidateOpts>,
    pub converters: MoovIoAchConverters,
}

impl MoovIoAchFileHeader {
    pub fn immediate_origin_field(&self) -> String {
        if self.immediate_origin.is_empty() {
            return " ".repeat(10);
        }
        let immediate_origin = self.immediate_origin.trim();
        if let Some(validate_opts) = &self.validate_opts {
            if validate_opts.bypass_origin_validation && immediate_origin.len() == 10 {
                return immediate_origin.to_string();
            }
        }
        " ".to_string() + &self.string_field(immediate_origin, 9)
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
        }

        // Pad with preallocated string
        let m = max - ln;
        let pad = moov_io_ach_string_zeros.get(&m).unwrap();
        pad.to_string() + s
    }
}

pub fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

#[derive(Debug)]
pub struct MoovIoAchValidateOpts {
    pub bypass_origin_validation: bool,
    pub bypass_destination_validation: bool,
}

#[derive(Debug)]
pub struct MoovIoAchConverters {}

lazy_static! {
    static ref moov_io_ach_string_zeros: HashMap<usize, String> = moov_io_ach_populate_map(94, "0");
}
