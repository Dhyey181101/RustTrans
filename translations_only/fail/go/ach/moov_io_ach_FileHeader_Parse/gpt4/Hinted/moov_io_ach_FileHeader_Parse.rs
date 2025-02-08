
use regex::Regex;
use std::time::SystemTime;

static MOOV_IO_ACH_HHMM_REGEX: &str = r"^([0-2]{1}[\d]{1}[0-5]{1}\d{1})$";

const MOOV_IO_ACH_RECORD_LENGTH: usize = 94;
const MOOV_IO_ACH_CHECKING_PRENOTE_CREDIT: u32 = 23;
const MOOV_IO_ACH_CHECKING_ZERO_DOLLAR_REMITTANCE_DEBIT: u32 = 29;
const MOOV_IO_ACH_CORRECTED_DATA_CHAR_LENGTH: u32 = 29;
const MOOV_IO_ACH_SAVINGS_PRENOTE_CREDIT: u32 = 33;
const MOOV_IO_ACH_SAVINGS_ZERO_DOLLAR_REMITTANCE_CREDIT: u32 = 34;
const MOOV_IO_ACH_FILE_HEADER_POS: &str = "1";
const MOOV_IO_ACH_DEBIT_FOR_DEBITS_REJECTED_BATCHES: u32 = 86;
const MOOV_IO_ACH_GLPRENOTE_DEBIT: u32 = 48;

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
    validate_opts: Option<Box<MoovIoAchValidateOpts>>,
}

struct MoovIoAchConverters;

struct MoovIoAchValidator;

struct MoovIoAchValidateOpts {
    preserve_spaces: bool,
}

impl MoovIoAchFileHeader {
    fn parse(&mut self, record: String) {
        if record.chars().count() != MOOV_IO_ACH_RECORD_LENGTH {
            return;
        }
        let runes: Vec<char> = record.chars().collect();
        self.priority_code = "01".to_string();
        self.immediate_destination = MoovIoAchConverters::parse_string_field(runes[3..13].iter().collect::<String>());
        self.immediate_origin = MoovIoAchConverters::parse_string_field(runes[13..23].iter().collect::<String>());
        self.file_creation_date = MoovIoAchValidator::validate_simple_date(runes[23..29].iter().collect::<String>());
        self.file_creation_time = MoovIoAchValidator::validate_simple_time(runes[29..33].iter().collect::<String>());
        self.file_id_modifier = runes[33..34].iter().collect::<String>();
        self.record_size = "094".to_string();
        self.blocking_factor = "10".to_string();
        self.format_code = "1".to_string();
        self.immediate_destination_name = MoovIoAchConverters::parse_string_field(runes[40..63].iter().collect::<String>());
        self.immediate_origin_name = MoovIoAchConverters::parse_string_field(runes[63..86].iter().collect::<String>());
        self.reference_code = MoovIoAchConverters::parse_string_field(runes[86..94].iter().collect::<String>());
    }
}

impl MoovIoAchConverters {
    fn parse_string_field(r: String) -> String {
        r.trim().to_string()
    }
}

impl MoovIoAchValidator {
    fn validate_simple_date(s: String) -> String {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(_) => s,
            Err(_) => "".to_string(),
        }
    }

    fn validate_simple_time(s: String) -> String {
        let re = Regex::new(MOOV_IO_ACH_HHMM_REGEX).unwrap();
        if re.is_match(&s) {
            s
        } else {
            "".to_string()
        }
    }
}
