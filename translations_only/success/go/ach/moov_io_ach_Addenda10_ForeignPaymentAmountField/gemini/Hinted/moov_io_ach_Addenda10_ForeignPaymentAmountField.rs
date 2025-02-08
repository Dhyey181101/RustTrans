
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub struct MoovIoAchAddenda10 {
    pub foreign_payment_amount: i64,
}

impl MoovIoAchAddenda10 {
    pub fn foreign_payment_amount_field(&self) -> String {
        self.numeric_field(self.foreign_payment_amount, 18)
    }

    fn numeric_field(&self, n: i64, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[(s.len() - max as usize)..].to_string();
        } else {
            let m = (max - s.len() as u32) as usize;
            let pad = STRING_ZEROS.get(&m).unwrap();
            return format!("{}{}", pad, s);
        }
    }
}

lazy_static! {
    static ref STRING_ZEROS: HashMap<usize, String> =
        (0..94).map(|i| (i, "0".repeat(i))).collect();
}
