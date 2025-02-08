
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
    pub fn parse(record: &str) -> Option<MoovIoAchAdvEntryDetail> {
        if record.len() != 94 {
            return None;
        }

        let runes = record.chars().collect::<Vec<char>>();

        let transaction_code = runes[1..3].iter().collect::<String>().parse::<i32>().ok()?;
        let rdfi_identification = runes[3..11].iter().collect::<String>();
        let check_digit = runes[11..12].iter().collect::<String>();
        let dfi_account_number = runes[12..27].iter().collect::<String>();
        let amount = runes[27..39].iter().collect::<String>().parse::<i32>().ok()?;
        let advice_routing_number = runes[39..48].iter().collect::<String>();
        let file_identification = runes[48..53].iter().collect::<String>();
        let ach_operator_data = runes[53..54].iter().collect::<String>();
        let individual_name = runes[54..76].iter().collect::<String>();
        let discretionary_data = runes[76..78].iter().collect::<String>();
        let addenda_record_indicator = runes[78..79].iter().collect::<String>().parse::<i32>().ok()?;
        let ach_operator_routing_number = runes[79..87].iter().collect::<String>();
        let julian_day = runes[87..90].iter().collect::<String>().parse::<i32>().ok()?;
        let sequence_number = runes[90..94].iter().collect::<String>().parse::<i32>().ok()?;

        Some(MoovIoAchAdvEntryDetail {
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
