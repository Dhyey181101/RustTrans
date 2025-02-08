
use regex::Regex;
use std::time::SystemTime;

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
        if record.chars().count() != 94 {
            return;
        }
        let runes: Vec<char> = record.chars().collect();

        self.priority_code = "01".to_string();
        self.immediate_destination = MoovIoAchConverters::parse_string_field_with_opts(
            runes[3..13].iter().collect(),
            &self.validate_opts,
        );
        self.immediate_origin = MoovIoAchConverters::parse_string_field_with_opts(
            runes[13..23].iter().collect(),
            &self.validate_opts,
        );
        self.file_creation_date = MoovIoAchValidator::validate_simple_date(
            runes[23..29].iter().collect(),
        );
        self.file_creation_time = MoovIoAchValidator::validate_simple_time(
            runes[29..33].iter().collect(),
        );
        self.file_id_modifier = runes[33..34].iter().collect();
        self.record_size = "094".to_string();
        self.blocking_factor = "10".to_string();
        self.format_code = "1".to_string();
        self.immediate_destination_name = MoovIoAchConverters::parse_string_field_with_opts(
            runes[40..63].iter().collect(),
            &self.validate_opts,
        );
        self.immediate_origin_name = MoovIoAchConverters::parse_string_field_with_opts(
            runes[63..86].iter().collect(),
            &self.validate_opts,
        );
        self.reference_code = MoovIoAchConverters::parse_string_field_with_opts(
            runes[86..94].iter().collect(),
            &self.validate_opts,
        );
    }
}

impl MoovIoAchConverters {
    fn parse_string_field(r: String) -> String {
        r.trim().to_string()
    }

    fn parse_string_field_with_opts(r: String, opts: &Option<Box<MoovIoAchValidateOpts>>) -> String {
        match opts {
            Some(opts) if opts.preserve_spaces => r,
            _ => Self::parse_string_field(r),
        }
    }
}

impl MoovIoAchValidator {
    fn validate_simple_date(s: String) -> String {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(_) => s, // Simplified for example, should actually parse and validate date
            Err(_) => "".to_string(),
        }
    }

    fn validate_simple_time(s: String) -> String {
        let re = Regex::new(r"^([0-2]{1}[\d]{1}[0-5]{1}\d{1})$").unwrap();
        if re.is_match(&s) {
            s
        } else {
            "".to_string()
        }
    }
}
