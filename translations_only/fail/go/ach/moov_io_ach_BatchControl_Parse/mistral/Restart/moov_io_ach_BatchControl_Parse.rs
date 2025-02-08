

use std::borrow::Borrow;
use std::str;
use std::string::String;
use std::num::ParseIntError;

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
    converters: MoovIoAchConverters,
    validate_opts: Option<Box<MoovIoAchValidateOpts>>,
}

impl MoovIoAchBatchControl {
    fn parse(&mut self, record: &str) {
        if record.len() != MOOV_IO_ACH_RECORD_LENGTH {
            return;
        }

        self.service_class_code = self.converters.parse_num_field(&record[0..3]);
        self.entry_addenda_count = self.converters.parse_num_field(&record[3..9]);
        self.entry_hash = self.converters.parse_num_field(&record[8..18]);
        self.total_debit_entry_dollar_amount = self.converters.parse_num_field(&record[17..27]);
        self.total_credit_entry_dollar_amount = self.converters.parse_num_field(&record[26..36]);
        self.company_identification = self.converters.parse_string_field_with_opts(&record[35..45], self.validate_opts.as_deref());
        self.message_authentication_code = self.converters.parse_string_field_with_opts(&record[44..63], self.validate_opts.as_deref());
        self.odfi_identification = self.converters.parse_string_field_with_opts(&record[72..79], self.validate_opts.as_deref());
        self.batch_number = self.converters.parse_num_field(&record[78..86]);
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn parse_num_field(&self, r: &str) -> i32 {
        match r.trim().parse::<i32>() {
            Ok(s) => s,
            Err(_) => 0,
        }
    }

    fn parse_string_field_with_opts(&self, r: &str, opts: Option<&MoovIoAchValidateOpts>) -> String {
        if opts.is_some() && opts.as_ref().unwrap().preserve_spaces {
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

