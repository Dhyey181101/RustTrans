
use std::str::FromStr;

const MOOV_IO_ACH_RECORDLENGTH: u32 = 94;
const MOOV_IO_ACH_CHECKINGDEBIT: u32 = 27;
const MOOV_IO_ACH_SAVINGSZERODOLLARREMITTANCEDEBIT: u32 = 39;
const MOOV_IO_ACH_GLPRENOTEDEBIT: u32 = 48;
const MOOV_IO_ACH_LOANPRENOTECREDIT: u32 = 53; 
const MOOV_IO_ACH_LOANZERODOLLARREMITTANCECREDIT: u32 = 54;
const MOOV_IO_ACH_CREDITSUMMARY: u32 = 87;

struct MoovIoAchAdvEntryDetail {
    transaction_code: i32,
    rdfi_identification: String,
    check_digit: String,
    dfi_account_number: String,
    amount: i32,
    advice_routing_number: String,
    file_identification: String,
    ach_operator_data: String,
    individual_name: String,
    discretionary_data: String,
    addenda_record_indicator: i32,
    ach_operator_routing_number: String,
    julian_day: i32,
    sequence_number: i32,
}

impl MoovIoAchAdvEntryDetail {
    fn parse(&mut self, record: &str) {
        if record.len() != MOOV_IO_ACH_RECORDLENGTH as usize {
            return;
        }
        
        let runes = record.as_bytes();
        let rdfi_slice = unsafe {
            std::str::from_utf8_unchecked(&runes[3..11])
        };
        
        self.transaction_code = self.parse_num_field(unsafe {
            std::str::from_utf8_unchecked(&runes[1..3])
        });
        self.rdfi_identification = self.parse_string_field(rdfi_slice);
        // ...
    }

    fn parse_num_field(&self, field: &str) -> i32 {
        let trimmed = field.trim();
        i32::from_str(trimmed).unwrap()
    }

    fn parse_string_field(&self, field: &str) -> String {
        let trimmed = field.trim();
        trimmed.to_string()
    }
}

struct MoovIoAchConverters;

