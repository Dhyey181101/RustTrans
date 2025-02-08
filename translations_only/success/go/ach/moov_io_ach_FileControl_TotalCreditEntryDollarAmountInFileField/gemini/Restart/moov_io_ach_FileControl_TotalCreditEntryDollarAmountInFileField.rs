
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct MoovIoAchFileControl {
    pub total_credit_entry_dollar_amount_in_file: i32,
}

pub struct MoovIoAchConverters {}

impl MoovIoAchFileControl {
    pub fn total_credit_entry_dollar_amount_in_file_field(&self) -> String {
        let s = self.total_credit_entry_dollar_amount_in_file.to_string();
        if s.len() > 12 {
            return s[(s.len() - 12)..].to_string();
        } else {
            let pad = MOOV_IO_ACH_STRING_ZEROS.get(&(12 - s.len())).unwrap();
            let pad = pad.to_string();
            return format!("{}{}", pad, s);
        }
    }
}

impl MoovIoAchConverters {}

lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, String> = {
        let mut out = HashMap::new();
        for i in 0..94 {
            out.insert(i, "0".repeat(i));
        }
        out
    };
}
