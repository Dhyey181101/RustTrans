
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug, Clone)]
pub struct MoovIoAchFileHeader {
    pub immediate_destination: String,
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
    pub converters: String,
}

impl MoovIoAchFileHeader {
    pub fn immediate_destination_field(&self) -> String {
        if self.immediate_destination.is_empty() {
            return " ".repeat(10);
        }
        let immediate_destination = self.immediate_destination.trim();
        if let Some(validate_opts) = &self.validate_opts {
            if validate_opts.bypass_destination_validation && immediate_destination.len() == 10 {
                return immediate_destination.to_string();
            }
        }
        format!(" {} {}", immediate_destination, self.string_field(immediate_destination, 9))
    }

    pub fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        if let Some(pad) = moov_io_ach_string_zeros.get(&m) {
            return format!("{}{}", pad, s);
        }

        "0".repeat(m) + s
    }
}

impl ToString for MoovIoAchFileHeader {
    fn to_string(&self) -> String {
        format!(
            "{}{}{}{}{}{}{}{}{}{}{}",
            self.immediate_destination_field(),
            self.immediate_origin,
            self.file_creation_date,
            self.file_creation_time,
            self.record_size,
            self.blocking_factor,
            self.format_code,
            self.immediate_destination_name,
            self.immediate_origin_name,
            self.reference_code,
            self.converters
        )
    }
}

#[derive(Debug, Clone)]
pub struct MoovIoAchValidateOpts {
    pub bypass_destination_validation: bool,
}

lazy_static! {
    static ref moov_io_ach_string_zeros: HashMap<usize, String> = {
        let mut out = HashMap::with_capacity(94);
        for i in 0..94 {
            out.insert(i, "0".repeat(i));
        }
        out
    };
}
