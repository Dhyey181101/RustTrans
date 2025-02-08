
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug)]
pub struct MoovIoAchAdvEntryDetail {
    pub advice_routing_number: String,
    pub file_identification: String,
    pub ach_operator_data: String,
    pub individual_name: String,
    pub discretionary_data: String,
    pub addenda_record_indicator: String,
    pub ach_operator_routing_number: String,
    pub julian_day: String,
    pub sequence_number: String,
    pub addenda_99: String,
    pub category: String,
    pub converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAdvEntryDetail {
    pub fn advice_routing_number_field(&self) -> String {
        self.converters.string_field(&self.advice_routing_number, 9)
    }
}

#[derive(Debug)]
pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as i32;
        if ln > max as i32 {
            return s[..max as usize].to_string();
        }

        let m = max as i32 - ln;
        let pad = moov_io_ach_string_zeros.get(&m).unwrap();
        pad.to_string() + s
    }
}

lazy_static! {
    static ref moov_io_ach_string_zeros: HashMap<i32, String> = {
        let mut out = HashMap::new();
        for i in 0..94 {
            out.insert(i, String::from_str(&"0".repeat(i as usize)).unwrap());
        }
        out
    };
}
