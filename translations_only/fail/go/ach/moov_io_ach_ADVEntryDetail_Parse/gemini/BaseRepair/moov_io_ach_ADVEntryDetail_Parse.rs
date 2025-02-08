
use std::str::FromStr;

#[derive(Debug)]
pub struct MoovIoAchAdvEntryDetail {
    pub transaction_code: i32,
    pub rdfi_identification: String,
    pub check_digit: String,
    pub dfi_account_number: String,
    pub amount: i32,
    pub advice_routing_number: String,
    pub file_identification: String,
    pub ach_operator_data: String,
    pub individual_name: String,
    pub discretionary_data: String,
    pub addenda_record_indicator: i32,
    pub ach_operator_routing_number: String,
    pub julian_day: i32,
    pub sequence_number: i32,
}

impl MoovIoAchAdvEntryDetail {
    pub fn parse(record: &str) -> Option<Self> {
        if record.len() != 94 {
            return None;
        }

        let runes = record.chars().collect::<Vec<_>>();

        // 1-1 Always "6"
        // 2-3 is checking credit 22 debit 27 savings credit 32 debit 37
        let transaction_code = runes[1..3].iter().collect::<String>().parse::<i32>().ok()?;
        // 4-11 the RDFI's routing number without the last digit.
        let rdfi_identification = runes[3..11].iter().collect::<String>();
        // 12-12 The last digit of the RDFI's routing number
        let check_digit = runes[11..12].iter().collect::<String>();
        // 13-27 The receiver's bank account number you are crediting/debiting
        let dfi_account_number = runes[12..27].iter().collect::<String>();
        // 28-39 Number of cents you are debiting/crediting this account
        let amount = runes[27..39].iter().collect::<String>().parse::<i32>().ok()?;
        // 40-48 Advice Routing Number
        let advice_routing_number = runes[39..48].iter().collect::<String>();
        // 49-53 File Identification
        let file_identification = runes[48..53].iter().collect::<String>();
        // 54-54 ACH Operator Data
        let ach_operator_data = runes[53..54].iter().collect::<String>();
        // 55-76 Individual Name
        let individual_name = runes[54..76].iter().collect::<String>();
        // 77-78 allows ODFIs to include codes of significance only to them, normally blank
        let discretionary_data = runes[76..78].iter().collect::<String>();
        // 79-79 1 if addenda exists 0 if it does not
        let addenda_record_indicator = runes[78..79].iter().collect::<String>().parse::<i32>().ok()?;
        // 80-87
        let ach_operator_routing_number = runes[79..87].iter().collect::<String>();
        // 88-90
        let julian_day = runes[87..90].iter().collect::<String>().parse::<i32>().ok()?;
        // 91-94
        let sequence_number = runes[90..94].iter().collect::<String>().parse::<i32>().ok()?;

        Some(Self {
            transaction_code,
            rdfi_identification,
            check_digit,
            dfi_account_number,
            amount,
            advice_routing_number,
            file_identification,
            ach_operator_data,
            individual_name,
            discretionary_data,
            addenda_record_indicator,
            ach_operator_routing_number,
            julian_day,
            sequence_number,
        })
    }
}
