

use std::borrow::Borrow;
use std::str;
use std::string::String;
use std::str::FromStr;

const MOOV_IO_ACH_RECORD_LENGTH: usize = 94;
const MOOV_IO_ACH_SAVINGS_CREDIT: usize = 32;
const MOOV_IO_ACH_GL_ZERO_DOLLAR_REMITTANCE_CREDIT: usize = 44;
const MOOV_IO_ACH_LOAN_ZERO_DOLLAR_REMITTANCE_CREDIT: usize = 54;
const MOOV_IO_ACH_CREDIT_SUMMARY: usize = 87;

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
    moov_io_ach_converters: Box<MoovIoAchConverters>,
    validate_opts: MoovIoAchValidateOpts,
}

impl MoovIoAchBatchControl {
    fn new() -> Self {
        MoovIoAchBatchControl {
            service_class_code: 0,
            entry_addenda_count: 0,
            entry_hash: 0,
            total_debit_entry_dollar_amount: 0,
            total_credit_entry_dollar_amount: 0,
            company_identification: String::new(),
            message_authentication_code: String::new(),
            odfi_identification: String::new(),
            batch_number: 0,
            moov_io_ach_converters: Box::new(MoovIoAchConverters),
            validate_opts: MoovIoAchValidateOpts::default(),
        }
    }

    fn parse(&mut self, record: &str) {
        if record.len() != MOOV_IO_ACH_RECORD_LENGTH {
            return;
        }

        self.service_class_code = self.moov_io_ach_converters.parse_num_field(&record[0..3]);
        self.entry_addenda_count = self.moov_io_ach_converters.parse_num_field(&record[3..9]);
        self.entry_hash = self.moov_io_ach_converters.parse_num_field(&record[8..18]);
        self.total_debit_entry_dollar_amount = self.moov_io_ach_converters.parse_num_field(&record[17..27]);
        self.total_credit_entry_dollar_amount = self.moov_io_ach_converters.parse_num_field(&record[26..36]);
        self.company_identification = self.moov_io_ach_converters.parse_string_field_with_opts(&record[35..45], &self.validate_opts);
        self.message_authentication_code = self.moov_io_ach_converters.parse_string_field_with_opts(&record[44..73], &self.validate_opts);
        self.odfi_identification = self.moov_io_ach_converters.parse_string_field_with_opts(&record[72..87], &self.validate_opts);
        self.batch_number = self.moov_io_ach_converters.parse_num_field(&record[86..94]);
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn parse_num_field(&self, r: &str) -> i32 {
        r.trim().parse::<i32>().unwrap_or(0)
    }

    fn parse_string_field_with_opts(&self, r: &str, opts: &MoovIoAchValidateOpts) -> String {
        if opts.preserve_spaces {
            r.to_string()
        } else {
            self.parse_string_field(r)
        }
    }

    fn parse_string_field(&self, r: &str) -> String {
        r.trim().to_string()
    }
}

#[derive(Default)]
struct MoovIoAchValidateOpts {
    preserve_spaces: bool,
}

