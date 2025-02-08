
use std::collections::HashMap;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct MoovIoAchValidateOpts {
    pub bypass_origin_validation: bool,
    pub bypass_destination_validation: bool,
    pub custom_trace_numbers: bool,
    pub allow_zero_batches: bool,
    pub allow_missing_file_header: bool,
    pub allow_missing_file_control: bool,
    pub bypass_company_identification_match: bool,
    pub custom_return_codes: bool,
    pub unequal_service_class_code: bool,
    pub allow_unordered_batch_numbers: bool,
    pub allow_invalid_check_digit: bool,
    pub unequal_addenda_counts: bool,
    pub preserve_spaces: bool,
    pub allow_invalid_amounts: bool,
}

#[derive(Debug, Clone)]
pub struct MoovIoAchConverters {}

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

        let m = max - ln;
        let pad = moov_io_ach_string_zeros.get(&m).unwrap();
        pad.to_string() + s
    }
}

lazy_static::lazy_static! {
    static ref moov_io_ach_string_zeros: HashMap<usize, String> =
        (0..94).map(|i| (i, "0".repeat(i))).collect();
}
