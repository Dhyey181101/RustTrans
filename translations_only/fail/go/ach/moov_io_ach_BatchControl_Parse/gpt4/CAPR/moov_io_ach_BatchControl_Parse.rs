
use std::str::FromStr;

struct MoovIoAchBatchControl {
    service_class_code: i32,
    entry_addenda_count: i32,
    entry_hash: i32,
    total_debit_entry_dollar_amount: i32,
    total_credit_entry_dollar_amount: i32,
    company_identification: String,
    message_authentication_code: String,
    odfi_identification: String,
    batch_number: i32,
    converters: MoovIoAchConverters,
    validate_opts: Option<Box<MoovIoAchValidateOpts>>,
}

struct MoovIoAchConverters;

struct MoovIoAchValidateOpts {
    preserve_spaces: bool,
}

impl MoovIoAchBatchControl {
    fn parse(&mut self, record: String) {
        if record.chars().count() != 94 {
            return;
        }

        self.service_class_code = self.converters.parse_num_field(&record[1..4]);
        self.entry_addenda_count = self.converters.parse_num_field(&record[4..10]);
        self.entry_hash = self.converters.parse_num_field(&record[10..20]);
        self.total_debit_entry_dollar_amount = self.converters.parse_num_field(&record[20..32]);
        self.total_credit_entry_dollar_amount = self.converters.parse_num_field(&record[32..44]);
        self.company_identification = self.converters.parse_string_field_with_opts(&record[44..54], &self.validate_opts);
        self.message_authentication_code = self.converters.parse_string_field_with_opts(&record[54..73], &self.validate_opts);
        self.odfi_identification = self.converters.parse_string_field_with_opts(&record[79..87], &self.validate_opts);
        self.batch_number = self.converters.parse_num_field(&record[87..94]);
    }
}

impl MoovIoAchConverters {
    fn parse_num_field(&self, r: &str) -> i32 {
        r.trim().parse::<i32>().unwrap_or(0)
    }

    fn parse_string_field_with_opts(&self, r: &str, opts: &Option<Box<MoovIoAchValidateOpts>>) -> String {
        match opts {
            Some(opts) if opts.preserve_spaces => r.to_string(),
            _ => self.parse_string_field(r),
        }
    }

    fn parse_string_field(&self, r: &str) -> String {
        r.trim().to_string()
    }
}
