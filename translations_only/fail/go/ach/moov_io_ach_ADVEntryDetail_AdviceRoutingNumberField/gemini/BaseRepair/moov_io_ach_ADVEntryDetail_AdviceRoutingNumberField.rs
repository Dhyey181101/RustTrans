
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct MoovIoAchAdvEntryDetail {
    pub advice_routing_number: String,
}

pub struct MoovIoAchConverters {}

impl MoovIoAchAdvEntryDetail {
    pub fn advice_routing_number_field(&self) -> &str {
        &self.advice_routing_number
    }
}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = max - ln;
        let pad = moov_io_ach_string_zeros.get(&(m as i32)).unwrap();
        format!("{}{}", pad, s)
    }
}

lazy_static! {
    static ref moov_io_ach_string_zeros: HashMap<i32, String> = {
        let mut out = HashMap::new();
        for i in 0..94 {
            out.insert(i, String::from_utf8(vec![b'0'; i as usize]).unwrap());
        }
        out
    };
}
