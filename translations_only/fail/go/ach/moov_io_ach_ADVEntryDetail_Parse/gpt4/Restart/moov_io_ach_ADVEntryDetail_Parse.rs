
use std::str::FromStr;

const MOOV_IO_ACH_RECORD_LENGTH: usize = 94;
const MOOV_IO_ACH_CHECKING_DEBIT: i32 = 27;
const MOOV_IO_ACH_SAVINGS_ZERO_DOLLAR_REMITTANCE_DEBIT: i32 = 39;
const MOOV_IO_ACH_GLPRENOTE_DEBIT: i32 = 48;
const MOOV_IO_ACH_LOANPRENOTE_CREDIT: i32 = 53;
const MOOV_IO_ACH_LOAN_ZERO_DOLLAR_REMITTANCE_CREDIT: i32 = 54;
const MOOV_IO_ACH_CREDIT_SUMMARY: i32 = 87;

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
    converters: MoovIoAchConverters,
}

impl MoovIoAchAdvEntryDetail {
    fn parse(&mut self, record: String) {
        if record.chars().count() != MOOV_IO_ACH_RECORD_LENGTH {
            return;
        }
        let chars: Vec<char> = record.chars().collect();

        self.transaction_code = self.converters.parse_num_field(chars[1..3].iter().collect());
        self.rdfi_identification = self.converters.parse_string_field(chars[3..11].iter().collect());
        self.check_digit = self.converters.parse_string_field(chars[11..12].iter().collect());
        self.dfi_account_number = chars[12..27].iter().collect();
        self.amount = self.converters.parse_num_field(chars[27..39].iter().collect());
        self.advice_routing_number = self.converters.parse_string_field(chars[39..48].iter().collect());
        self.file_identification = self.converters.parse_string_field(chars[48..53].iter().collect());
        self.ach_operator_data = self.converters.parse_string_field(chars[53..54].iter().collect());
        self.individual_name = chars[54..76].iter().collect();
        self.discretionary_data = chars[76..78].iter().collect();
        self.addenda_record_indicator = self.converters.parse_num_field(chars[78..79].iter().collect());
        self.ach_operator_routing_number = self.converters.parse_string_field(chars[79..87].iter().collect());
        self.julian_day = self.converters.parse_num_field(chars[87..90].iter().collect());
        self.sequence_number = self.converters.parse_num_field(chars[90..94].iter().collect());
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn parse_num_field(&self, r: String) -> i32 {
        r.trim().parse::<i32>().unwrap_or(0)
    }

    fn parse_string_field(&self, r: String) -> String {
        r.trim().to_string()
    }
}

fn main() {}
