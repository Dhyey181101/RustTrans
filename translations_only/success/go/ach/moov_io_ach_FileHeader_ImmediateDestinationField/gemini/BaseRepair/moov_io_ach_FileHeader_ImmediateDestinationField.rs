
use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;
use std::string::ToString;
use lazy_static::lazy_static;

#[derive(Debug)]
pub struct MoovIoAchFileHeader {
    pub id: String,
    pub priority_code: String,
    pub immediate_destination: String,
    pub immediate_origin: String,
    pub file_creation_date: String,
    pub file_creation_time: String,
    pub file_id_modifier: String,
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
        " ".to_string() + &self.string_field(immediate_destination, 9)
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
        }

        // Pad with preallocated string
        let m = max - ln;
        if let Some(pad) = moov_io_ach_string_zeros.get(&m) {
            return pad.to_string() + s;
        }
        // slow path
        "0".repeat(m) + s
    }
}

#[derive(Debug)]
pub struct MoovIoAchValidateOpts {
    pub skip_all: bool,
    pub require_aba_origin: bool,
    pub bypass_origin_validation: bool,
    pub bypass_destination_validation: bool,
    pub check_transaction_code: bool,
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

#[derive(Debug)]
pub struct MoovIoAchConverters {}

lazy_static! {
    static ref moov_io_ach_string_zeros: HashMap<usize, String> = {
        let mut out = HashMap::with_capacity(94);
        for i in 0..94 {
            out.insert(i, "0".repeat(i));
        }
        out
    };
}
