


use regex::Regex;
use std::time::{Duration, SystemTime};

const RECORD_LENGTH: usize = 94;
const CHECKING_PRENOTE_CREDIT: u8 = 23;
const CHECKING_ZERO_DOLLAR_REMITTANCE_DEBIT: u8 = 29;
const CORRECTED_DATA_CHAR_LENGTH: u8 = 29;
const SAVINGS_PRENOTE_CREDIT: u8 = 33;
const SAVINGS_ZERO_DOLLAR_REMITTANCE_CREDIT: u8 = 34;
const FILE_HEADER_POS: &str = "1";
const DEBIT_FOR_DEBITS_REJECTED_BATCHES: u8 = 86;
const GL_PRENOTE_DEBIT: u8 = 48;

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
    validator: MoovIoAchValidator,
    converters: MoovIoAchConverters,
    validate_opts: Option<Box<MoovIoAchValidateOpts>>,
}

impl MoovIoAchFileHeader {
    fn parse(&mut self, record: &str) {
        if record.len() != RECORD_LENGTH {
            return;
        }
        let runes: Vec<char> = record.chars().collect();

        // (character position 1-1) Always "1"
        // (2-3) Always "01"
        self.priority_code = "01".to_string();
        // (4-13) A blank space followed by your ODFI's routing number. For example: " 121140399"
        self.immediate_destination = self.trim_routing_number_leading_zero(&runes[2..13].iter().collect::<String>());
        // (14-23) A 10-digit number assigned to you by the ODFI once they approve you to originate ACH files through them
        self.immediate_origin = self.trim_routing_number_leading_zero(&runes[13..23].iter().collect::<String>());
        // 24-29 Today's date in YYMMDD format
        // must be after today's date.
        self.file_creation_date = self.validate_simple_date(&runes[22..29].iter().collect::<String>()).unwrap().to_string();
        // 30-33 The current time in HHmm format
        self.file_creation_time = self.validate_simple_time(&runes[29..34].iter().collect::<String>()).unwrap().to_string();
        // 35-37 Always "A"
        self.file_id_modifier = runes[34].to_string();
        // 35-37 always "094"
        self.record_size = "094".to_string();
        // 38-39 always "10"
        self.blocking_factor = "10".to_string();
        // 40 always "1"
        self.format_code = "1".to_string();
        // 41-63 The name of the ODFI. example "SILICON VALLEY BANK    "
        self.immediate_destination_name = self.parse_string_field_with_opts(&runes[40..63].iter().collect::<String>(), &self.validate_opts);
        // 64-86 ACH operator or sending point that is sending the file
        self.immediate_origin_name = self.parse_string_field_with_opts(&runes[63..86].iter().collect::<String>(), &self.validate_opts);
        // 87-94 Optional field that may be used to describe the ACH file for internal accounting purposes
        self.reference_code = self.parse_string_field_with_opts(&runes[86..].iter().collect::<String>(), &self.validate_opts);
    }

    fn parse_string_field(&self, r: String) -> String {
        r.trim().to_string()
    }

    fn trim_routing_number_leading_zero(
        &self,
        r: &str
    ) -> String {
        r.trim_start_matches('0')
            .to_string()
    }

    fn validate_simple_date(
        &self,
        r: &str
    ) -> Option<String> {
        let re = Regex::new(r"^(19|20)\d\d(0[1-9]|1[012])(0[1-9]|[12][0-9]|3[01])$").unwrap();
        if re.is_match(r) {
            let date_str = format!("{}-{}-{}", &r[0..2], &r[2..4], &r[4..6]);
            Some(date_str)
        } else {
            None
        }
    }

    fn validate_simple_time(
        &self,
        r: &str
    ) -> Option<String> {
        let re = Regex::new(r"^(0[0-9]|1[0-9]|2[0-3])\d$").unwrap();
        if re.is_match(r) {
            let time_str = format!("{}:00", r);
            Some(time_str)
        } else {
            None
        }
    }

    fn parse_string_field_with_opts(
        &self,
        r: &str,
        _validate_opts: &Option<Box<MoovIoAchValidateOpts>>
    ) -> String {
        r.trim().to_string()
    }
}

struct MoovIoAchValidator;
struct MoovIoAchConverters;
struct MoovIoAchValidateOpts;


