
use std::collections::HashMap;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug, Clone)]
pub struct MoovIoAchIATEntryDetail {
    pub rdfi_identification: String,
    pub check_digit: String,
}

impl MoovIoAchIATEntryDetail {
    pub fn set_rdfi(&mut self, rdfi: &str) -> &mut Self {
        let s = MoovIoAchConverters {}.string_field(rdfi, 9);
        self.rdfi_identification = MoovIoAchConverters {}.parse_string_field(&s[..8]);
        self.check_digit = MoovIoAchConverters {}.parse_string_field(&s[8..9]);
        self
    }
}

pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
        }

        // Pad with preallocated string
        let m = max - ln;
        let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap();
        format!("{}{}", pad, s)
    }

    pub fn parse_string_field(&self, r: &str) -> String {
        r.trim().to_string()
    }
}

lazy_static::lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, String> = {
        let mut out = HashMap::new();
        for i in 0..94 {
            out.insert(i, "0".repeat(i));
        }
        out
    };
}
