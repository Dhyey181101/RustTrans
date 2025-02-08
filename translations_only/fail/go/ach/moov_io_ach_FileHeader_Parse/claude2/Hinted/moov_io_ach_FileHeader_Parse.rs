
use std::str;
use once_cell::sync::Lazy;
use regex::Regex;

static MOOV_IO_ACH_HHMM_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^([0-2]{1}[\d]{1}[0-5]{1}\d{1})$").unwrap());

const MOOV_IO_ACH_RECORD_LENGTH: usize = 94;
const MOOV_IO_ACH_CHECKING_PRENOTE_CREDIT: usize = 23;
const MOOV_IO_ACH_CHECKING_ZERO_DOLLAR_REMITTANCE_DEBIT: usize = 29;
const MOOV_IO_ACH_CORRECTED_DATA_CHAR_LENGTH: usize = 29;
const MOOV_IO_ACH_SAVINGS_PRENOTE_CREDIT: usize = 33;
const MOOV_IO_ACH_SAVINGS_ZERO_DOLLAR_REMITTANCE_CREDIT: usize = 34;
const MOOV_IO_ACH_FILE_HEADER_POS: &str = "1";
const MOOV_IO_ACH_DEBIT_FOR_DEBITS_REJECTED_BATCHES: usize = 86;
const MOOV_IO_ACH_GL_PRENOTE_DEBIT: usize = 48;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn parse_string_field(&self, r: &str) -> String {
        r.trim().to_string()
    }
}

fn moov_io_ach_trim_routing_number_leading_zero(s: &str) -> String {
    if s.len() == 10 && &s[..1] == "0" && s != "0000000000" {
        s[1..].trim().to_string()
    } else {
        s.trim().to_string()
    }
}

struct MoovIoAchValidator;

impl MoovIoAchValidator {
    fn validate_simple_date(&self, s: &str) -> String {
        s.to_string()
    }

    fn validate_simple_time(&self, s: &str) -> String {
        if MOOV_IO_ACH_HHMM_REGEX.is_match(s) {
            s.to_string()
        } else {
            String::new()
        }
    }
}

impl MoovIoAchConverters {
    fn parse_string_field_with_opts(
        &self,
        r: &str,
        opts: &Option<Box<MoovIoAchValidateOpts>>,
    ) -> String {
        match opts {
            Some(o) if o.preserve_spaces => r.to_string(),
            _ => self.parse_string_field(r),
        }
    }
}

struct MoovIoAchValidateOpts {
    preserve_spaces: bool,
}

struct MoovIoAchFileHeader {
    priority_code: String,
    immediate_destination: String,
    immediate_origin: String,
    file_creation_date: String,
    file_creation_time: String,
    file_id_modifier: String,
    record_size: String,
    blocking_factor: String,
    format_code: String,
    immediate_destination_name: String,
    immediate_origin_name: String,
    reference_code: Option<String>,
    validator: MoovIoAchValidator,
    converters: MoovIoAchConverters,
    validate_opts: Option<Box<MoovIoAchValidateOpts>>,
}

impl MoovIoAchFileHeader {
    fn parse(&mut self, record: &str) {
        if record.len() != MOOV_IO_ACH_RECORD_LENGTH {
            return;
        }
        
        let record = record.as_bytes();

        self.priority_code = "01".to_string();
            
        self.immediate_destination = self
            .converters
            .parse_string_field(&moov_io_ach_trim_routing_number_leading_zero(
                str::from_utf8(&record[3..13]).unwrap(),
            ));
        
        self.immediate_origin = self
            .converters
            .parse_string_field(&moov_io_ach_trim_routing_number_leading_zero(
                str::from_utf8(&record[13..23]).unwrap(),
            ));
        
        self.file_creation_date = self
            .validator
            .validate_simple_date(str::from_utf8(&record[23..29]).unwrap());
        
        self.file_creation_time = self
            .validator
            .validate_simple_time(str::from_utf8(&record[29..33]).unwrap());

        self.file_id_modifier = str::from_utf8(&record[33..34]).unwrap().to_string();
        
        self.record_size = "094".to_string();
        
        self.blocking_factor = "10".to_string();
        
        self.format_code = "1".to_string();
        
        self.immediate_destination_name = self
            .converters
            .parse_string_field_with_opts(
                str::from_utf8(&record[40..63]).unwrap(),
                &self.validate_opts,
            );
        
        self.immediate_origin_name = self
            .converters
            .parse_string_field_with_opts(
                str::from_utf8(&record[63..86]).unwrap(),
                &self.validate_opts,
            );
        
        self.reference_code = Some(
            self.converters
                .parse_string_field_with_opts(str::from_utf8(&record[86..94]).unwrap(), &self.validate_opts)
        );
    }
}

