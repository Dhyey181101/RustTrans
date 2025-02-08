

use std::str;
use std::borrow::Cow;

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

        self.transaction_code = self.parse_num_field(&runes[0..3].iter().collect::<String>());
        self.rdfi_identification = self.parse_string_field(&runes[2..11].iter().collect::<String>());
        self.check_digit = self.parse_string_field(&runes[10..11].iter().collect::<String>());
        self.dfi_account_number = self.parse_string_field(&runes[11..27].iter().collect::<String>());
        self.amount = self.parse_num_field(&runes[26..39].iter().collect::<String>());
        self.advice_routing_number = self.parse_string_field(&runes[38..48].iter().collect::<String>());
        self.file_identification = self.parse_string_field(&runes[47..53].iter().collect::<String>());
        self.ach_operator_data = " ".to_string(); //self.parse_string_field(&runes[52..53].iter().collect::<String>());
        self.individual_name = self.parse_string_field(&runes[53..76].iter().collect::<String>());
        self.discretionary_data = "  ".to_string(); //self.parse_string_field(&runes[75..77].iter().collect::<String>());
        self.addenda_record_indicator = self.parse_num_field(&runes[76..77].iter().collect::<String>());
        self.ach_operator_routing_number = self.parse_string_field(&runes[77..87].iter().collect::<String>());
        self.julian_day = self.parse_num_field(&runes[86..89].iter().collect::<String>());
        self.sequence_number = self.parse_num_field(&runes[89..94].iter().collect::<String>());
    }

    fn parse_num_field(&self, r: &str) -> i32 {
        match r.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => 0,
        }
    }

    fn parse_string_field(&self, r: &str) -> String {
        String::from(r.trim())
    }
}

