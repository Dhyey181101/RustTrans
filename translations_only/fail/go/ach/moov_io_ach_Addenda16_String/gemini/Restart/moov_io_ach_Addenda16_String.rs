
use std::collections::HashMap;
use std::fmt::Write;

const MOOV_IO_ACH_RECORD_LENGTH: usize = 94;
const MOOV_IO_ACH_ENTRY_ADDENDA_POS: &str = "7";

#[derive(Debug)]
pub struct MoovIoAchAddenda16 {
    pub type_code: String,
    pub receiver_city_state_province: String,
    pub receiver_country_postal_code: String,
    pub entry_detail_sequence_number: i32,
}

impl MoovIoAchAddenda16 {
    pub fn new(
        type_code: String,
        receiver_city_state_province: String,
        receiver_country_postal_code: String,
        entry_detail_sequence_number: i32,
    ) -> Self {
        Self {
            type_code,
            receiver_city_state_province,
            receiver_country_postal_code,
            entry_detail_sequence_number,
        }
    }

    pub fn to_string(&self) -> String {
        let mut buf = String::with_capacity(MOOV_IO_ACH_RECORD_LENGTH);
        write!(
            &mut buf,
            "{}{}{}{}{}{}",
            MOOV_IO_ACH_ENTRY_ADDENDA_POS,
            self.type_code,
            self.receiver_city_state_province_field(),
            self.receiver_country_postal_code_field(),
            "              ",
            self.entry_detail_sequence_number_field()
        )
        .unwrap();
        buf
    }

    fn receiver_city_state_province_field(&self) -> String {
        self.alpha_field(&self.receiver_city_state_province, 35)
    }

    fn alpha_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = " ".repeat(m);
            format!("{}{}", s, pad)
        }
    }

    fn receiver_country_postal_code_field(&self) -> String {
        self.alpha_field(&self.receiver_country_postal_code, 35)
    }

    fn entry_detail_sequence_number_field(&self) -> String {
        self.numeric_field(self.entry_detail_sequence_number, 7)
    }

    fn numeric_field(&self, n: i32, max: usize) -> String {
        let s = n.to_string();
        if s.len() > max {
            s[s.len() - max..].to_string()
        } else {
            let m = max - s.len();
            let pad = "0".repeat(m);
            format!("{}{}", pad, s)
        }
    }
}

