
use std::collections::HashMap;

struct MoovIoAchIatEntryDetail {
    rdfi_identification: String,
    check_digit: String,
}

impl MoovIoAchIatEntryDetail {
    fn set_rdfi(&mut self, rdfi: &str, converters: &MoovIoAchConverters) -> &mut Self {
        let s = converters.string_field(rdfi, 9);
        self.rdfi_identification = converters.parse_string_field(&s[..8]);
        self.check_digit = converters.parse_string_field(&s[8..9]);
        self
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let m = max - ln;
            let pad = get_zeros(m as usize);
            format!("{}{}", pad, s)
        }
    }

    fn parse_string_field(&self, s: &str) -> String {
        s.trim().to_string()
    }
}

fn get_zeros(len: usize) -> String {
    "0".repeat(len)
}

