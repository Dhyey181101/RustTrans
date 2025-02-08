
use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;
use std::string::ToString;

#[derive(Clone)]
pub struct MoovIoAchFileHeader {
    pub id: Option<String>,
    pub priority_code: Option<String>,
    pub immediate_destination: Option<String>,
    pub immediate_origin: Option<String>,
    pub file_creation_date: Option<String>,
    pub file_creation_time: Option<String>,
    pub file_id_modifier: Option<String>,
    pub record_size: Option<String>,
    pub blocking_factor: Option<String>,
    pub format_code: Option<String>,
    pub immediate_destination_name: Option<String>,
    pub immediate_origin_name: Option<String>,
    pub reference_code: Option<String>,
    pub validate_opts: Option<MoovIoAchValidateOpts>,
    pub converters: MoovIoAchConverters,
}

impl MoovIoAchFileHeader {
    pub fn immediate_origin_field(&self) -> String {
        if self.immediate_origin.is_none() {
            return " ".repeat(10);
        }
        let immediate_origin = self.immediate_origin.as_ref().unwrap();
        let immediate_origin = immediate_origin.trim();
        if let Some(validate_opts) = &self.validate_opts {
            if validate_opts.bypass_origin_validation && immediate_origin.len() == 10 {
                return immediate_origin.to_string();
            }
        }
        format!(" {} {}", immediate_origin, self.string_field(immediate_origin, 9))
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
        }

        // Pad with preallocated string
        let m = max - ln;
        let pad = moov_io_ach_string_zeros.get(&m).unwrap();
        format!("{}{}", pad, s)
    }
}

impl fmt::Display for MoovIoAchFileHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            self.id.as_ref().unwrap_or(&"".to_string()),
            self.priority_code.as_ref().unwrap_or(&"".to_string()),
            self.immediate_destination.as_ref().unwrap_or(&"".to_string()),
            self.immediate_origin_field(),
            self.file_creation_date.as_ref().unwrap_or(&"".to_string()),
            self.file_creation_time.as_ref().unwrap_or(&"".to_string()),
            self.file_id_modifier.as_ref().unwrap_or(&"".to_string()),
            self.record_size.as_ref().unwrap_or(&"".to_string()),
            self.blocking_factor.as_ref().unwrap_or(&"".to_string()),
            self.format_code.as_ref().unwrap_or(&"".to_string()),
            self.immediate_destination_name.as_ref().unwrap_or(&"".to_string()),
            self.immediate_origin_name.as_ref().unwrap_or(&"".to_string()),
            self.reference_code.as_ref().unwrap_or(&"".to_string()),
            self.validate_opts.as_ref().unwrap_or(&MoovIoAchValidateOpts::default()).to_string(),
            self.converters.to_string(),
            "\n",
            "\n",
            "\n"
        )
    }
}

#[derive(Clone)]
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

impl Default for MoovIoAchValidateOpts {
    fn default() -> Self {
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

impl ToString for MoovIoAchValidateOpts {
    fn to_string(&self) -> String {
        format!(
            "{{skip_all: {}, require_aba_origin: {}, bypass_origin_validation: {}, bypass_destination_validation: {}, check_transaction_code: {}, custom_trace_numbers: {}, allow_zero_batches: {}, allow_missing_file_header: {}, allow_missing_file_control: {}, bypass_company_identification_match: {}, custom_return_codes: {}, unequal_service_class_code: {}, allow_unordered_batch_numbers: {}, allow_invalid_check_digit: {}, unequal_addenda_counts: {}, preserve_spaces: {}, allow_invalid_amounts: {}}}",
            self.skip_all,
            self.require_aba_origin,
            self.bypass_origin_validation,
            self.bypass_destination_validation,
            self.check_transaction_code,
            self.custom_trace_numbers,
            self.allow_zero_batches,
            self.allow_missing_file_header,
            self.allow_missing_file_control,
            self.bypass_company_identification_match,
            self.custom_return_codes,
            self.unequal_service_class_code,
            self.allow_unordered_batch_numbers,
            self.allow_invalid_check_digit,
            self.unequal_addenda_counts,
            self.preserve_spaces,
            self.allow_invalid_amounts
        )
    }
}

#[derive(Clone)]
pub struct MoovIoAchConverters {}

impl ToString for MoovIoAchConverters {
    fn to_string(&self) -> String {
        "{}".to_string()
    }
}

lazy_static::lazy_static! {
    static ref moov_io_ach_string_zeros: HashMap<usize, String> = {
        let mut map = HashMap::new();
        for i in 0..94 {
            map.insert(i, "0".repeat(i));
        }
        map
    };
}

