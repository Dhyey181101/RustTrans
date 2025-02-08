

use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct MoovIoAchAdvEntryDetail {
    pub advice_routing_number: String,
}

impl MoovIoAchAdvEntryDetail {
    pub fn advice_routing_number_field(&self) -> String {
        self.advice_routing_number.clone()
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
        let pad = moov_io_ach_string_zeros.get(&(m as i32)).unwrap();
        return format!("{}{}", pad, s);
    }
}

lazy_static! {
    static ref moov_io_ach_string_zeros: HashMap<i32, String> = {
        let mut out = HashMap::new();
        for i in 0..94 {
            out.insert(i, String::from("0").repeat(i as usize));
        }
        out
    };
}

