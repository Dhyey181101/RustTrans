
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::Write;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct MoovIoAchEntryDetail {
    pub identification_number: String,
}

impl MoovIoAchEntryDetail {
    pub fn set_shr_document_reference_number(&mut self, s: &str) {
        self.identification_number.push_str(&self.string_field(s, 11));
    }

    fn string_field(&self, s: &str, max: u32) -> String {
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
            out.insert(i, "0".repeat(i as usize));
        }
        out
    };
}
