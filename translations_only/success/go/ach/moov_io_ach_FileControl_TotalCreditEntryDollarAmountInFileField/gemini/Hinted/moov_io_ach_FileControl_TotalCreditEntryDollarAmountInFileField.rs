

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub struct MoovIoAchFileControl {
    pub total_credit_entry_dollar_amount_in_file: i32,
}

impl MoovIoAchFileControl {
    pub fn total_credit_entry_dollar_amount_in_file_field(&self) -> String {
        self.numeric_field(self.total_credit_entry_dollar_amount_in_file, 12)
    }
}

impl MoovIoAchFileControl {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[(s.len() - max as usize)..].to_string();
        } else {
            let m = max - s.len() as u32;
            let pad = MOOv_IO_ACH_STRING_ZEROS.get(&m).unwrap();
            return format!("{}{}", pad, s);
        }
    }
}

lazy_static! {
    static ref MOOv_IO_ACH_STRING_ZEROS: HashMap<u32, String> = {
        let mut out = HashMap::new();
        for i in 0..94 {
            out.insert(i, "0".repeat(i as usize));
        }
        out
    };
}

