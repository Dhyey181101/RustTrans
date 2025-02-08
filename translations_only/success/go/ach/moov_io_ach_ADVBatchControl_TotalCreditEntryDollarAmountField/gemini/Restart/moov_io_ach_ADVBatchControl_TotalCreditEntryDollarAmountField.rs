
use std::collections::HashMap;

pub struct MoovIoAchAdvBatchControl {
    pub total_credit_entry_dollar_amount: i32,
}

impl MoovIoAchAdvBatchControl {
    pub fn total_credit_entry_dollar_amount_field(&self) -> String {
        MoovIoAchConverters::new().numeric_field(self.total_credit_entry_dollar_amount, 20)
    }
}

pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn new() -> Self {
        Self {}
    }

    pub fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() > max as usize {
            return s[s.len() - max as usize..].to_string();
        } else {
            let m = max as usize - s.len();
            let pad = moov_io_ach_populate_map(m as i32, "0");
            return format!("{}{}", pad, s);
        }
    }
}

fn moov_io_ach_populate_map(max: i32, zero: &str) -> String {
    zero.repeat(max as usize)
}
