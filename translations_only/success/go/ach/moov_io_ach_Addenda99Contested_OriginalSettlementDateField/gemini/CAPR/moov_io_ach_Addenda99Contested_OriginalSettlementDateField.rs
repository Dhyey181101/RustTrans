
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct MoovIoAchAddenda99Contested {
    pub original_settlement_date: String,
}

impl MoovIoAchAddenda99Contested {
    pub fn original_settlement_date_field(&self) -> String {
        MoovIoAchConverters::string_field(&self.original_settlement_date, 3)
    }
}

pub struct MoovIoAchConverters;

impl MoovIoAchConverters {
    pub fn string_field(s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = max - ln;
        let pad = &moov_io_ach_string_zeros[&m];
        format!("{}{}", pad, s)
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
