
use std::collections::HashMap;
use std::fmt::Write;
use std::str::FromStr;
use lazy_static::lazy_static;

const MOOV_IO_ACH_RECORD_LENGTH: usize = 94;
const MOOV_IO_ACH_ENTRY_ADDENDA_POS: &str = "7";

#[derive(Debug, Default)]
pub struct MoovIoAchAddenda12 {
    pub type_code: String,
    pub originator_city_state_province: String,
    pub originator_country_postal_code: String,
    pub entry_detail_sequence_number: i32,
}

impl MoovIoAchAddenda12 {
    pub fn to_string(&self) -> String {
        let mut buf = String::with_capacity(MOOV_IO_ACH_RECORD_LENGTH);
        write!(buf, "{}", MOOV_IO_ACH_ENTRY_ADDENDA_POS).unwrap();
        write!(buf, "{}", self.type_code).unwrap();
        write!(buf, "{}", self.originator_city_state_province_field()).unwrap();
        write!(buf, "{}", self.originator_country_postal_code_field()).unwrap();
        write!(buf, "{: >20}", "").unwrap();
        write!(buf, "{}", self.entry_detail_sequence_number_field()).unwrap();
        buf
    }

    pub fn originator_city_state_province_field(&self) -> String {
        let ln = self.originator_city_state_province.chars().count();
        if ln > 35 {
            self.originator_city_state_province[..35].to_string()
        } else {
            let m = 35 - ln;
            let pad = SPACE_ZEROS.get(&m).unwrap();
            format!("{}{}", self.originator_city_state_province, pad)
        }
    }

    pub fn originator_country_postal_code_field(&self) -> String {
        let ln = self.originator_country_postal_code.chars().count();
        if ln > 35 {
            self.originator_country_postal_code[..35].to_string()
        } else {
            let m = 35 - ln;
            let pad = SPACE_ZEROS.get(&m).unwrap();
            format!("{}{}", self.originator_country_postal_code, pad)
        }
    }

    pub fn entry_detail_sequence_number_field(&self) -> String {
        let s = self.entry_detail_sequence_number.to_string();
        if s.len() > 7 {
            s[s.len() - 7..].to_string()
        } else {
            let m = 7 - s.len();
            let pad = STRING_ZEROS.get(&m).unwrap();
            format!("{}{}", pad, s)
        }
    }
}

lazy_static! {
    static ref SPACE_ZEROS: HashMap<usize, String> =
        (0..=94).map(|i| (i, " ".repeat(i))).collect();
    static ref STRING_ZEROS: HashMap<usize, String> =
        (0..=94).map(|i| (i, "0".repeat(i))).collect();
}
