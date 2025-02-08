
use std::collections::HashMap;

pub struct MoovIoAchIatBatchHeader {
    pub foreign_exchange_reference_indicator: i32,
    pub foreign_exchange_reference: String,
    pub iso_destination_country_code: String,
    pub originator_identification: String,
    pub standard_entry_class_code: String,
    pub company_entry_description: String,
    pub iso_originating_currency_code: String,
    pub iso_destination_currency_code: String,
    pub effective_entry_date: String,
    pub settlement_date: String,
    pub originator_status_code: i32,
    pub odfi_identification: String,
    pub batch_number: i32,
}

pub struct MoovIoAchConverters {}

impl MoovIoAchIatBatchHeader {
    pub fn foreign_exchange_reference_indicator_field(&self) -> String {
        MoovIoAchConverters {}.numeric_field(self.foreign_exchange_reference_indicator, 1)
    }
}

impl MoovIoAchConverters {
    pub fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[(s.len() - max as usize)..].to_string();
        } else {
            let m = max - s.len() as u32;
            let pad = moov_io_ach_populate_map(m as i32, "0");
            return pad + &s;
        }
    }
}

fn moov_io_ach_populate_map(max: i32, zero: &str) -> String {
    let mut out = String::new();
    for i in 0..max {
        out.push_str(&zero.repeat(i as usize));
    }
    out
}
