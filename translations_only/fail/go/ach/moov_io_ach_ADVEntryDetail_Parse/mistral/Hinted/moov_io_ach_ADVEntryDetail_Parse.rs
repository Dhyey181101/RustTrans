

use std::borrow::Cow;
use std::str;
use std::string::String;

const MOOV_IO_ACH_RECORD_LENGTH: usize = 94;
const MOOV_IO_ACH_CHECKING_CREDIT: u8 = 22;
const MOOV_IO_ACH_CHECKING_DEBIT: u8 = 27;
const MOOV_IO_ACH_SAVINGS_CREDIT: u8 = 32;
const MOOV_IO_ACH_SAVINGS_ZERO_DOLLAR_REMITTANCE_DEBIT: u8 = 37;
const MOOV_IO_ACH_GL_PRENOTE_DEBIT: u8 = 48;
const MOOV_IO_ACH_LOAN_PRENOTE_CREDIT: u8 = 53;
const MOOV_IO_ACH_LOAN_ZERO_DOLLAR_REMITTANCE_CREDIT: u8 = 54;
const MOOV_IO_ACH_CREDIT_SUMMARY: u8 = 87;

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
        if record.len() != MOOV_IO_ACH_RECORD_LENGTH {
            return;
        }
        let runes: Vec<char> = record.chars().collect();

        self.transaction_code = self.parse_num_field(&runes[0..3]);
        self.rdfi_identification = self.parse_string_field(&runes[2..11]);
        self.check_digit = self.parse_string_field(&runes[10..11]);
        self.dfi_account_number = self.parse_string_field(&runes[11..27]);
        self.amount = self.parse_num_field(&runes[26..39]);
        self.advice_routing_number = self.parse_string_field(&runes[38..48]);
        self.file_identification = self.parse_string_field(&runes[47..53]);
        self.ach_operator_data = self.parse_string_field(&runes[52..53]);
        self.individual_name = self.parse_string_field(&runes[53..76]);
        self.discretionary_data = self.parse_string_field(&runes[75..77]);
        self.addenda_record_indicator = self.parse_num_field(&runes[76..77]);
        self.ach_operator_routing_number = self.parse_string_field(&runes[77..87]);
        self.julian_day = self.parse_num_field(&runes[86..89]);
        self.sequence_number = self.parse_num_field(&runes[89..94]);
    }

    fn parse_string_field(&self, r: &[char]) -> String {
        let s: Cow<str> = Cow::Owned(r.iter().collect::<String>());
        s.trim().to_string()
    }

    fn parse_num_field(&self, r: &[char]) -> i32 {
        let s: Cow<str> = Cow::Owned(r.iter().collect::<String>());
        s.trim().parse::<i32>().unwrap_or(0)
    }
}

