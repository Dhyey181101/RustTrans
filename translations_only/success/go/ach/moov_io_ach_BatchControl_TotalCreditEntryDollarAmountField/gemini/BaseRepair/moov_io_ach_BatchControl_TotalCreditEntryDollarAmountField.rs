
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct MoovIoAchBatchControl {
    pub total_credit_entry_dollar_amount: i32,
}

impl MoovIoAchBatchControl {
    pub fn total_credit_entry_dollar_amount_field(&self) -> String {
        MoovIoAchConverters {}.numeric_field(self.total_credit_entry_dollar_amount, 12)
    }
}

pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[(s.len() - max as usize)..].to_string();
        } else {
            let m = max - s.len() as u32;
            let pad = moov_io_ach_string_zeros.get(&m).unwrap();
            return format!("{}{}", pad, s);
        }
    }
}

lazy_static! {
    static ref moov_io_ach_string_zeros: HashMap<u32, String> =
        (0..94).map(|i| (i, "0".repeat(i as usize))).collect();
}
