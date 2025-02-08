
use std::str::FromStr;

#[derive(Debug)]
pub struct BatchControl {
    pub service_class_code: i32,
    pub entry_addenda_count: i32,
    pub entry_hash: i32,
    pub total_debit_entry_dollar_amount: i32,
    pub total_credit_entry_dollar_amount: i32,
    pub company_identification: String,
    pub message_authentication_code: String,
    pub odfi_identification: String,
    pub batch_number: i32,
}

impl BatchControl {
    pub fn parse(record: &str) -> Option<Self> {
        if record.len() != 94 {
            return None;
        }

        Some(BatchControl {
            service_class_code: i32::from_str(&record[1..4]).unwrap_or(0),
            entry_addenda_count: i32::from_str(&record[4..10]).unwrap_or(0),
            entry_hash: i32::from_str(&record[10..20]).unwrap_or(0),
            total_debit_entry_dollar_amount: i32::from_str(&record[20..32]).unwrap_or(0),
            total_credit_entry_dollar_amount: i32::from_str(&record[32..44]).unwrap_or(0),
            company_identification: record[44..54].trim().to_string(),
            message_authentication_code: record[54..73].trim().to_string(),
            odfi_identification: record[79..87].trim().to_string(),
            batch_number: i32::from_str(&record[87..94]).unwrap_or(0),
        })
    }
}
