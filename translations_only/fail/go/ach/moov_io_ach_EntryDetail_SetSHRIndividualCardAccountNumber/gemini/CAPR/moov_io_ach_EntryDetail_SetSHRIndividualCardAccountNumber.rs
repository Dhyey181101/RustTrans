
use std::collections::HashMap;
use std::ops::Deref;
use std::str::FromStr;
use std::string::ToString;
use lazy_static::lazy_static;

#[derive(Debug)]
pub struct MoovIoAchEntryDetail {
    pub individual_name: String,
}

impl MoovIoAchEntryDetail {
    pub fn set_shr_individual_card_account_number(&mut self, s: &str) {
        self.individual_name = s.to_string();
    }
}

#[derive(Debug)]
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
            out.insert(i, String::from_str("0").unwrap().repeat(i as usize));
        }
        out
    };
}

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}
