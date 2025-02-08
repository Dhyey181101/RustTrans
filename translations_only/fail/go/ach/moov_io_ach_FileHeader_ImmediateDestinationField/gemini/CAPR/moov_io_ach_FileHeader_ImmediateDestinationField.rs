
use lazy_static::lazy_static;
use std::collections::HashMap;
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
    pub converters: MoovIoAchConverters,
}

#[derive(Debug, Clone)]
pub struct MoovIoAchValidateOpts {
    pub bypass_destination_validation: bool,
}

#[derive(Debug, Clone)]
pub struct MoovIoAchConverters {}

impl MoovIoAchFileHeader {
    pub fn immediate_destination_field(&self) -> String {
        if self.immediate_destination.is_empty() {
            return " ".repeat(10);
        }
        let trimmed_immediate_destination = self.immediate_destination.trim();
        if let Some(validate_opts) = &self.validate_opts {
            if validate_opts.bypass_destination_validation && trimmed_immediate_destination.len() == 10 {
                return trimmed_immediate_destination.to_string();
            }
        }
        format!(" {} {}", trimmed_immediate_destination, self.string_field(&trimmed_immediate_destination, 9))
    }

    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = max - ln;
        if let Some(pad) = moov_io_ach_string_zeros.get(&(m as usize)) {
            return format!("{}{}", pad, s);
        }

        "0".repeat(m as usize) + s
    }
}

lazy_static! {
    static ref moov_io_ach_string_zeros: HashMap<usize, String> =
        (0..94).map(|i| (i, "0".repeat(i))).collect();
}
