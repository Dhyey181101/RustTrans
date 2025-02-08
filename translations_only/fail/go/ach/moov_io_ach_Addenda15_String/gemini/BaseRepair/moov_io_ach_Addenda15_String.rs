
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::Write;
use std::str::FromStr;

const MOOV_IO_ACH_RECORD_LENGTH: usize = 94;
const MOOV_IO_ACH_ENTRY_ADDENDA_POS: &str = "7";

#[derive(Debug, Default)]
pub struct MoovIoAchAddenda15 {
    pub type_code: String,
    pub receiver_id_number: Option<String>,
    pub receiver_street_address: String,
    pub entry_detail_sequence_number: i32,
}

impl MoovIoAchAddenda15 {
    pub fn to_string(&self) -> String {
        let mut buf = String::with_capacity(MOOV_IO_ACH_RECORD_LENGTH);
        write!(buf, "{}", MOOV_IO_ACH_ENTRY_ADDENDA_POS).unwrap();
        write!(buf, "{}", self.type_code).unwrap();
        write!(buf, "{}", self.receiver_id_number.as_ref().unwrap_or(&"".to_string())).unwrap();
        write!(buf, "{}", self.receiver_street_address_field()).unwrap();
        write!(buf, "{: >35}", "").unwrap();
        write!(buf, "{}", self.entry_detail_sequence_number_field()).unwrap();
        buf
    }

    pub fn receiver_street_address_field(&self) -> String {
        self.alpha_field(&self.receiver_street_address, 35)
    }

    pub fn entry_detail_sequence_number_field(&self) -> String {
        self.numeric_field(self.entry_detail_sequence_number, 7)
    }

    fn alpha_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = SPACE_ZEROS.get(&m).unwrap();
            format!("{}{}", s, pad)
        }
    }

    fn numeric_field(&self, n: i32, max: usize) -> String {
        let s = n.to_string();
        let l = s.chars().count();
        if l > max {
            s[l - max..].to_string()
        } else {
            let m = max - l;
            let pad = STRING_ZEROS.get(&m).unwrap();
            format!("{}{}", pad, s)
        }
    }
}

lazy_static! {
    static ref SPACE_ZEROS: HashMap<usize, String> = (0..=94)
        .map(|i| (i, " ".repeat(i)))
        .collect();
    static ref STRING_ZEROS: HashMap<usize, String> = (0..=94)
        .map(|i| (i, "0".repeat(i)))
        .collect();
}
