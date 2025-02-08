
use std::str::FromStr;

const MOOV_IO_ACH_RECORD_LENGTH: usize = 94;
const MOOV_IO_ACH_CHECKING_DEBIT: i32 = 27;
const MOOV_IO_ACH_SAVINGS_ZERO_DOLLAR_REMITTANCE_DEBIT: i32 = 39;
const MOOV_IO_ACH_GLPRENOTE_DEBIT: i32 = 48;
const MOOV_IO_ACH_LOANPRENOTE_CREDIT: i32 = 53;
const MOOV_IO_ACH_LOAN_ZERO_DOLLAR_REMITTANCE_CREDIT: i32 = 54;
const MOOV_IO_ACH_CREDIT_SUMMARY: i32 = 87;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn parse_num_field(&self, r: &str) -> i32 {
        r.trim().parse::<i32>().unwrap_or(0)
    }

    fn parse_string_field(&self, r: &str) -> String {
        r.trim().to_string()
    }
}

struct MoovIoAchADVEntryDetail {
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
    converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchADVEntryDetail {
    fn new() -> Self {
        MoovIoAchADVEntryDetail {
            transaction_code: 0,
            rdfi_identification: String::new(),
            check_digit: String::new(),
            dfi_account_number: String::new(),
            amount: 0,
            advice_routing_number: String::new(),
            file_identification: String::new(),
            ach_operator_data: String::new(),
            individual_name: String::new(),
            discretionary_data: String::new(),
            addenda_record_indicator: 0,
            ach_operator_routing_number: String::new(),
            julian_day: 0,
            sequence_number: 0,
            converters: Box::new(MoovIoAchConverters),
        }
    }

    fn parse(&mut self, record: &str) {
        if record.chars().count() != MOOV_IO_ACH_RECORD_LENGTH {
            return;
        }
        let runes: Vec<char> = record.chars().collect();

        self.transaction_code = self.converters.parse_num_field(&runes[1..3].iter().collect::<String>());
        self.rdfi_identification = self.converters.parse_string_field(&runes[3..11].iter().collect::<String>());
        self.check_digit = self.converters.parse_string_field(&runes[11..12].iter().collect::<String>());
        self.dfi_account_number = runes[12..27].iter().collect();
        self.amount = self.converters.parse_num_field(&runes[27..39].iter().collect::<String>());
        self.advice_routing_number = self.converters.parse_string_field(&runes[39..48].iter().collect::<String>());
        self.file_identification = self.converters.parse_string_field(&runes[48..53].iter().collect::<String>());
        self.ach_operator_data = self.converters.parse_string_field(&runes[53..54].iter().collect::<String>());
        self.individual_name = runes[54..76].iter().collect();
        self.discretionary_data = runes[76..78].iter().collect();
        self.addenda_record_indicator = self.converters.parse_num_field(&runes[78..79].iter().collect::<String>());
        self.ach_operator_routing_number = self.converters.parse_string_field(&runes[79..87].iter().collect::<String>());
        self.julian_day = self.converters.parse_num_field(&runes[87..90].iter().collect::<String>());
        self.sequence_number = self.converters.parse_num_field(&runes[90..94].iter().collect::<String>());
    }
}
