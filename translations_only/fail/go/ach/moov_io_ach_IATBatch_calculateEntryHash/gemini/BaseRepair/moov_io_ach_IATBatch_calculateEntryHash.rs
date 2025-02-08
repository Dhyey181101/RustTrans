
use std::convert::TryInto;

#[derive(Debug)]
pub struct IATBatch {
    pub entries: Vec<IATEntryDetail>,
}

impl IATBatch {
    pub fn calculate_entry_hash(&self) -> i32 {
        let mut hash = 0;
        for entry in &self.entries {
            let rdfi = entry.rdfi_identification.parse::<i32>().unwrap();
            hash += rdfi;
        }

        hash % 10_i32.pow(10)
    }
}

#[derive(Debug)]
pub struct IATEntryDetail {
    pub transaction_code: i32,
    pub rdfi_identification: String,
    pub amount: i32,
    pub dfia_account_number: String,
    pub trace_number: String,
}

pub fn aba8(rtn: &str) -> String {
    match rtn.len() {
        10 => {
            if rtn.chars().next().unwrap() == '0' || rtn.chars().next().unwrap() == '1' {
                rtn[1..9].to_string()
            } else {
                "".to_string()
            }
        }
        8 | 9 => rtn[..8].to_string(),
        _ => "".to_string(),
    }
}

pub fn least_significant_digits(v: i32, max_digits: u32) -> i32 {
    v % 10_i32.pow(max_digits as u32)
}
