
use lazy_static::lazy_static;

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
    pub addenda99: String,
    pub category: String,
    pub converters: MoovIoAchConverters,
}

pub struct MoovIoAchConverters {}

impl MoovIoAchAdvEntryDetail {
    pub fn advice_routing_number_field(&self) -> String {
        self.converters.string_field(&self.advice_routing_number, 9)
    }
}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = max - ln;
        let pad = moov_io_ach_string_zeros.get(m as usize).unwrap();
        format!("{}{}", pad, s)
    }
}

lazy_static! {
    static ref moov_io_ach_string_zeros: Vec<String> = {
        let mut out = Vec::new();
        for i in 0..94 {
            out.push("0".repeat(i as usize));
        }
        out
    };
}
