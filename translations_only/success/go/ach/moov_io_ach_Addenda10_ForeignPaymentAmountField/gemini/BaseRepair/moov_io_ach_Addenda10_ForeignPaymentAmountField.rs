
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug)]
pub struct MoovIoAchAddenda10 {
    pub foreign_payment_amount: i32,
}

impl MoovIoAchAddenda10 {
    pub fn foreign_payment_amount_field(&self) -> String {
        self.numeric_field(self.foreign_payment_amount, 18)
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s[(l - max) as usize..].to_string();
        } else {
            let m = max - l;
            let pad = moov_io_ach_string_zeros.get(&m).unwrap();
            return format!("{}{}", pad, s);
        }
    }
}

lazy_static! {
    static ref moov_io_ach_string_zeros: HashMap<u32, String> =
        (0..94).map(|i| (i, String::from_str("0").unwrap().repeat(i as usize))).collect();
}
