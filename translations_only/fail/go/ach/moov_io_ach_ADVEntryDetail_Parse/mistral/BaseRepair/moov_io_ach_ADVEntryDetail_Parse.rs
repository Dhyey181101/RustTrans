
use std::str;
use std::borrow::Cow;
use std::num::ParseIntError;

const RECORD_LENGTH: usize = 94;
const CHECKING_CREDIT: usize = 22;
const CHECKING_DEBIT: usize = 27;
const SAVINGS_CREDIT: usize = 32;
const SAVINGS_ZERO_DOLLAR_REMITTANCE_DEBIT: usize = 37;
const GL_PRENOTE_DEBIT: usize = 48;
const LOAN_PRENOTE_CREDIT: usize = 53;
const LOAN_ZERO_DOLLAR_REMITTANCE_CREDIT: usize = 54;
const CREDIT_SUMMARY: usize = 87;

struct ADVEntryDetail {
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

impl ADVEntryDetail {
    fn parse(&mut self, record: &str) {
        if record.len() != RECORD_LENGTH {
            return;
        }

        let runes = record.chars();

        self.transaction_code = self.parse_num_field(runes.clone().take(3).collect::<String>());
        self.rdfi_identification = self.parse_string_field(runes.clone().skip(2).take(9).collect::<String>());
        self.check_digit = self.parse_string_field(runes.clone().skip(11).take(1).collect::<String>());
        self.dfi_account_number = runes.clone().skip(12).take(15).collect::<String>();
        self.amount = self.parse_num_field(runes.clone().skip(27).take(12).collect::<String>());
        self.advice_routing_number = self.parse_string_field(runes.clone().skip(39).take(8).collect::<String>());
        self.file_identification = self.parse_string_field(runes.clone().skip(48).take(5).collect::<String>());
        self.ach_operator_data = self.parse_string_field(runes.clone().skip(53).take(1).collect::<String>());
        self.individual_name = self.parse_string_field(runes.clone().skip(54).take(22).collect::<String>());
        self.discretionary_data = self.parse_string_field(runes.clone().skip(76).take(2).collect::<String>());
        self.addenda_record_indicator = self.parse_num_field(runes.clone().skip(78).take(1).collect::<String>());
        self.ach_operator_routing_number = self.parse_string_field(runes.clone().skip(79).take(8).collect::<String>());
        self.julian_day = self.parse_num_field(runes.clone().skip(87).take(3).collect::<String>());
        self.sequence_number = self.parse_num_field(runes.skip(90).take(4).collect::<String>());
    }

    fn parse_num_field(&self, r: String) -> i32 {
        match r.trim().parse::<i32>() {
            Ok(n) => n,
            Err(_) => 0,
        }
    }

    fn parse_string_field(&self, r: String) -> String {
        r.trim().to_string()
    }
}
