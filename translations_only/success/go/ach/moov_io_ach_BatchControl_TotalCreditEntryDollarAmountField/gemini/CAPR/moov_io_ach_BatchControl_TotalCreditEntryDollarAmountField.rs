
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct MoovIoAchBatchControl {
    pub total_credit_entry_dollar_amount: i32,
}

pub struct MoovIoAchConverters {}

impl MoovIoAchBatchControl {
    pub fn total_credit_entry_dollar_amount_field(&self) -> String {
        let converters = MoovIoAchConverters {};
        converters.numeric_field(self.total_credit_entry_dollar_amount, 12)
    }
}

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
    static ref moov_io_ach_string_zeros: HashMap<u32, String> = {
        let mut out = HashMap::new();
        for i in 0..94 {
            out.insert(i, "0".repeat(i as usize));
        }
        out
    };
}
