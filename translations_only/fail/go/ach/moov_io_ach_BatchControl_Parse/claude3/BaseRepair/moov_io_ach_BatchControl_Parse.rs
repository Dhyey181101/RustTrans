
use std::str;

const MOOV_IO_ACH_RECORD_LENGTH: usize = 94;
const MOOV_IO_ACH_SAVINGS_CREDIT: usize = 32;
const MOOV_IO_ACH_GLZERO_DOLLAR_REMITTANCE_CREDIT: usize = 44;
const MOOV_IO_ACH_LOANZERO_DOLLAR_REMITTANCE_CREDIT: usize = 54;
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
}

impl MoovIoAchBatchControl {
    fn parse(&mut self, record: &str) {
        if record.len() != MOOV_IO_ACH_RECORD_LENGTH {
            return;
        }

        self.service_class_code = self.parse_num_field(&record[1..4]);
        self.entry_addenda_count = self.parse_num_field(&record[4..10]);
        self.entry_hash = self.parse_num_field(&record[10..20]);
        self.total_debit_entry_dollar_amount = self.parse_num_field(&record[20..32]);
        self.total_credit_entry_dollar_amount = self.parse_num_field(&record[32..44]);
        self.company_identification = self.parse_string_field_with_opts(&record[44..54], false);
        self.message_authentication_code = self.parse_string_field_with_opts(&record[54..73], false);
        self.odfi_identification = self.parse_string_field_with_opts(&record[79..87], false);
        self.batch_number = self.parse_num_field(&record[87..94]);
    }

    fn parse_num_field(&self, r: &str) -> i32 {
        r.trim().parse().unwrap_or(0)
    }

    fn parse_string_field_with_opts(&self, r: &str, preserve_spaces: bool) -> String {
        if preserve_spaces {
            r.to_string()
        } else {
            self.parse_string_field(r)
        }
    }

    fn parse_string_field(&self, r: &str) -> String {
        r.trim().to_string()
    }
}
