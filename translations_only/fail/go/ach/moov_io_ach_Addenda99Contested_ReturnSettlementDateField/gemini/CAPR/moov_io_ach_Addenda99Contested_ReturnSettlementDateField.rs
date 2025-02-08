
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct MoovIoAchAddenda99Contested {
    pub return_settlement_date: String,
}

impl MoovIoAchAddenda99Contested {
    pub fn return_settlement_date(&self) -> &str {
        &self.return_settlement_date
    }
}

pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = max - ln;
        let pad = moov_io_ach_string_zeros.get(&m).unwrap();
        format!("{}{}", pad, s)
    }
}

lazy_static! {
    static ref moov_io_ach_string_zeros: HashMap<u32, String> = {
        let mut out = HashMap::new();
        for i in 0..94 {
            out.insert(i, String::from_str("0").unwrap().repeat(i as usize));
        }
        out
    };
}
