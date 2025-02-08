
#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::str;

const MOOV_IO_ACH_RECORD_LENGTH: usize = 94;
const MOOV_IO_ACH_CHECKING_PRENOTE_CREDIT: u8 = 23;
const MOOV_IO_ACH_CHECKING_ZERO_DOLLAR_REMITTANCE_DEBIT: u8 = 29;
const MOOV_IO_ACH_CORRECTED_DATA_CHAR_LENGTH: usize = 29;
const MOOV_IO_ACH_SAVINGS_PRENOTE_CREDIT: u8 = 33;
const MOOV_IO_ACH_SAVINGS_ZERO_DOLLAR_REMITTANCE_CREDIT: u8 = 34;
const MOOV_IO_ACH_FILE_HEADER_POS: &str = "1";
const MOOV_IO_ACH_DEBIT_FOR_DEBITS_REJECTED_BATCHES: u8 = 86;
const MOOV_IO_ACH_GL_PRENOTE_DEBIT: u8 = 48;

lazy_static! {
    static ref MOOV_IO_ACH_HHMMREGEX: Regex = Regex::new(r"^([0-2]{1}[\d]{1}[0-5]{1}\d{1})$").unwrap();
}

fn moov_io_ach_trim_routing_number_leading_zero(s: &str) -> String {
    if s.len() == 10 && s.starts_with('0') && s != "0000000000" {
        s[1..].trim().to_string()
    } else {
        s.trim().to_string()
    }
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
    reference_code: String,
    validate_opts: Box<MoovIoAchValidateOpts>,
}

impl MoovIoAchFileHeader {
    fn parse(&mut self, record: &str) {
        if record.len() != MOOV_IO_ACH_RECORD_LENGTH {
            return;
        }

        self.priority_code = "01".to_string();
        self.immediate_destination = moov_io_ach_trim_routing_number_leading_zero(&record[3..13]);
        self.immediate_origin = moov_io_ach_trim_routing_number_leading_zero(&record[13..23]);
        self.file_creation_date = self.validate_simple_date(&record[23..29]);
        self.file_creation_time = self.validate_simple_time(&record[29..33]);
        self.file_id_modifier = record[33..34].to_string();
        self.record_size = "094".to_string();
        self.blocking_factor = "10".to_string();
        self.format_code = "1".to_string();
        self.immediate_destination_name = self.parse_string_field_with_opts(&record[40..63], &self.validate_opts);
        self.immediate_origin_name = self.parse_string_field_with_opts(&record[63..86], &self.validate_opts);
        self.reference_code = self.parse_string_field_with_opts(&record[86..94], &self.validate_opts);
    }

    fn parse_string_field(&self, r: &str) -> String {
        r.trim().to_string()
    }

    fn validate_simple_date(&self, s: &str) -> String {
        match str::parse::<u32>(&s[..2]) {
            Ok(year) if year >= 0 && year <= 99 => {}
            _ => return "".to_string(),
        }

        match str::parse::<u32>(&s[2..4]) {
            Ok(month) if month >= 1 && month <= 12 => {}
            _ => return "".to_string(),
        }

        match str::parse::<u32>(&s[4..6]) {
            Ok(day) if day >= 1 && day <= 31 => {}
            _ => return "".to_string(),
        }

        s.to_string()
    }

    fn validate_simple_time(&self, s: &str) -> String {
        if MOOV_IO_ACH_HHMMREGEX.is_match(s) {
            s.to_string()
        } else {
            "".to_string()
        }
    }

    fn parse_string_field_with_opts(&self, r: &str, opts: &Box<MoovIoAchValidateOpts>) -> String {
        if opts.preserve_spaces {
            r.to_string()
        } else {
            self.parse_string_field(r)
        }
    }
}

struct MoovIoAchValidateOpts {
    preserve_spaces: bool,
}

fn main() {}
