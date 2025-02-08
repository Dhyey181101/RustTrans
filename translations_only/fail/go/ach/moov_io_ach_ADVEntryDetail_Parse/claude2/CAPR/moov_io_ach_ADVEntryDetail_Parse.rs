

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
    file_identification: Option<String>,
    ach_operator_data: Option<String>,
    individual_name: String,
    discretionary_data: Option<String>,
    addenda_record_indicator: Option<i32>,
    ach_operator_routing_number: String,
    julian_day: i32,
    sequence_number: i32,
}

impl MoovIoAchAdvEntryDetail {
    fn parse(&mut self, record: &str) {
        if record.len() != MOOV_IO_ACH_RECORDLENGTH as usize {
            return;
        }
        
        let runes = record.chars().collect::<Vec<_>>();
        
        // Parse each field
        self.transaction_code = parse_num_field(&runes[1..3].iter().collect::<String>());
        self.rdfi_identification = parse_string_field(&runes[3..11].iter().collect::<String>());
        self.check_digit = parse_string_field(&runes[11..12].iter().collect::<String>());
        self.dfi_account_number = runes[12..27].iter().collect::<String>();
        self.amount = parse_num_field(&runes[27..39].iter().collect::<String>());
        self.advice_routing_number = parse_string_field(&runes[39..48].iter().collect::<String>());
        self.file_identification = Some(parse_string_field(&runes[48..53].iter().collect::<String>()));
        self.ach_operator_data = Some(parse_string_field(&runes[53..54].iter().collect::<String>()));
        self.individual_name = runes[54..76].iter().collect::<String>();
        self.discretionary_data = Some(runes[76..78].iter().collect::<String>());
        self.addenda_record_indicator = Some(parse_num_field(&runes[78..79].iter().collect::<String>()));
        self.ach_operator_routing_number = parse_string_field(&runes[79..87].iter().collect::<String>());
        self.julian_day = parse_num_field(&runes[87..90].iter().collect::<String>());
        self.sequence_number = parse_num_field(&runes[90..94].iter().collect::<String>());
    }
}

fn parse_num_field(r: &str) -> i32 {
    r.trim().parse::<i32>().unwrap()
}

fn parse_string_field(r: &str) -> String {
    r.trim().to_string()
}

struct MoovIoAchConverters;

