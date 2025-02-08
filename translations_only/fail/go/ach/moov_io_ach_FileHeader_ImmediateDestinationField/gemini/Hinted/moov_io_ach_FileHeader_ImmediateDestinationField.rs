
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
        format!(" {} {}", immediate_destination, self.string_field(immediate_destination, 9))
    }

    pub fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
        }

        // Pad with preallocated string
        let m = max - ln;
        if let Some(pad) = moov_io_ach_string_zeros.get(&m) {
            return format!("{}{}", pad, s);
        }
        // slow path
        format!("{:0>max$}", s)
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
            self.file_id_modifier,
            self.record_size,
            self.blocking_factor,
            self.format_code,
            self.immediate_destination_name,
            self.immediate_origin_name,
            self.reference_code
        )
    }
}

#[derive(Debug, Clone)]
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

impl MoovIoAchValidateOpts {
    pub fn new() -> Self {
        Self {
            skip_all: false,
            require_aba_origin: false,
            bypass_origin_validation: false,
            bypass_destination_validation: false,
            check_transaction_code: false,
            custom_trace_numbers: false,
            allow_zero_batches: false,
            allow_missing_file_header: false,
            allow_missing_file_control: false,
            bypass_company_identification_match: false,
            custom_return_codes: false,
            unequal_service_class_code: false,
            allow_unordered_batch_numbers: false,
            allow_invalid_check_digit: false,
            unequal_addenda_counts: false,
            preserve_spaces: false,
            allow_invalid_amounts: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn new() -> Self {
        Self {}
    }
}

lazy_static::lazy_static! {
    static ref moov_io_ach_string_zeros: HashMap<usize, String> = {
        let mut out = HashMap::new();
        for i in 0..94 {
            out.insert(i, "0".repeat(i));
        }
        out
    };
}
