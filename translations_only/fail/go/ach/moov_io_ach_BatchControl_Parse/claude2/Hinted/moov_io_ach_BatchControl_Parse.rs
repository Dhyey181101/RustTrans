
use std::str::FromStr;

const MOOV_IO_ACH_RECORDLENGTH: usize = 94;
const MOOV_IO_ACH_SAVINGSCREDIT: usize = 32;
const MOOV_IO_ACH_GLZERODOLLARREMITTANCECREDIT: usize = 44;
const MOOV_IO_ACH_LOANZERODOLLARREMITTANCECREDIT: usize = 54;
const MOOV_IO_ACH_CREDITSUMMARY: usize = 87;

struct MoovIoAchBatchControl {
    service_class_code: u32,
    entry_addenda_count: u32,
    entry_hash: u32,
    total_debit_entry_dollar_amount: u32,
    total_credit_entry_dollar_amount: u32,
    company_identification: String,
    message_authentication_code: String,
    odfi_identification: String,
    batch_number: u32,
    moov_io_ach_converters: MoovIoAchConverters,
    validate_opts: Option<MoovIoAchValidateOpts>,
}

impl MoovIoAchBatchControl {
    fn parse(&mut self, record: &str) {
        if record.len() != MOOV_IO_ACH_RECORDLENGTH {
            return;
        }
        
        self.service_class_code = self.parse_num_field(&record[1..4]);
        
        self.entry_addenda_count = self.parse_num_field(&record[4..10]);
        
        self.entry_hash = self.parse_num_field(&record[10..20]);
        
        self.total_debit_entry_dollar_amount = self.parse_num_field(&record[20..32]);
        
        self.total_credit_entry_dollar_amount = self.parse_num_field(&record[32..44]);

        self.company_identification = self.parse_string_field_with_opts(&record[44..54], self.validate_opts.as_ref());

        self.message_authentication_code = self.parse_string_field_with_opts(&record[54..73], self.validate_opts.as_ref());
        
        self.odfi_identification = self.parse_string_field_with_opts(&record[79..87], self.validate_opts.as_ref());
        
        self.batch_number = self.parse_num_field(&record[87..94]);
    }

    fn parse_num_field(&self, record: &str) -> u32 {
        record.trim().parse::<u32>().unwrap_or(0)
    }
    
    fn parse_string_field_with_opts(&self, record: &str, opts: Option<&MoovIoAchValidateOpts>) -> String {
        if let Some(opts) = opts {
            if opts.preserve_spaces {
                return record.to_string();
            }
        }
        
        self.parse_string_field(record)
    }

    fn parse_string_field(&self, record: &str) -> String {
        record.trim().to_string()
    }
}

struct MoovIoAchConverters;

struct MoovIoAchValidateOpts {
    preserve_spaces: bool,
}
