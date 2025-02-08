
use std::convert::TryInto;

#[derive(Debug)]
pub struct MoovIoAchIatBatch {
    pub entries: Vec<MoovIoAchIateEntryDetail>,
    pub converters: MoovIoAchConverters,
}

impl MoovIoAchIatBatch {
    pub fn calculate_entry_hash(&self) -> i32 {
        let mut hash = 0;
        for entry in &self.entries {
            let rdfi_identification = entry.rdfi_identification.clone();
            let rdfi_identification = rdfi_identification.chars().take(8).collect::<String>();
            let rdfi = rdfi_identification.parse::<i32>().unwrap();
            hash += rdfi;
        }

        self.least_significant_digits(hash, 10)
    }

    fn least_significant_digits(&self, v: i32, max_digits: u32) -> i32 {
        v % (10_i32.pow(max_digits))
    }
}

#[derive(Debug)]
pub struct MoovIoAchIateEntryDetail {
    pub transaction_code: String,
    pub rdfi_identification: String,
    pub amount: i64,
    pub dfia_account_number: String,
    pub trace_number: String,
    pub addenda10: String,
    pub addenda11: String,
    pub addenda12: String,
    pub addenda13: String,
    pub addenda14: String,
    pub addenda15: String,
    pub addenda16: String,
    pub addenda17: String,
    pub addenda18: String,
    pub addenda98: String,
    pub addenda99: String,
    pub converters: MoovIoAchConverters,
}

#[derive(Debug)]
pub struct MoovIoAchConverters {}
