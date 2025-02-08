
use std::collections::HashMap;
use std::fmt::Write;
use std::str::FromStr;
use lazy_static::lazy_static;

const MOO_IO_ACH_RECORD_LENGTH: usize = 94;
const MOO_IO_ACH_ENTRY_ADDENDA_POS: &str = "7";

#[derive(Debug)]
pub struct MoovIoAchAddenda17 {
    pub type_code: String,
    pub payment_related_information: String,
    pub sequence_number: i32,
    pub entry_detail_sequence_number: i32,
}

impl MoovIoAchAddenda17 {
    pub fn new(
        type_code: String,
        payment_related_information: String,
        sequence_number: i32,
        entry_detail_sequence_number: i32,
    ) -> Self {
        Self {
            type_code,
            payment_related_information,
            sequence_number,
            entry_detail_sequence_number,
        }
    }

    pub fn to_string(&self) -> String {
        let mut buf = String::with_capacity(MOO_IO_ACH_RECORD_LENGTH);
        write!(buf, "{}", MOO_IO_ACH_ENTRY_ADDENDA_POS).unwrap();
        write!(buf, "{}", self.type_code).unwrap();
        write!(buf, "{}", self.payment_related_information_field()).unwrap();
        write!(buf, "{}", self.sequence_number_field()).unwrap();
        write!(buf, "{}", self.entry_detail_sequence_number_field()).unwrap();
        buf
    }

    pub fn payment_related_information_field(&self) -> String {
        let pad = SPACE_ZEROS.get(&80).unwrap();
        let ln = self.payment_related_information.chars().count();
        if ln > 80 {
            self.payment_related_information[..80].to_string()
        } else {
            let m = 80 - ln;
            format!("{}{}", self.payment_related_information, &pad[..m])
        }
    }

    pub fn sequence_number_field(&self) -> String {
        let pad = STRING_ZEROS.get(&4).unwrap();
        let s = self.sequence_number.to_string();
        if s.len() > 4 {
            s[s.len() - 4..].to_string()
        } else {
            let m = 4 - s.len();
            format!("{}{}", &pad[..m], s)
        }
    }

    pub fn entry_detail_sequence_number_field(&self) -> String {
        let pad = STRING_ZEROS.get(&7).unwrap();
        let s = self.entry_detail_sequence_number.to_string();
        if s.len() > 7 {
            s[s.len() - 7..].to_string()
        } else {
            let m = 7 - s.len();
            format!("{}{}", &pad[..m], s)
        }
    }
}

lazy_static! {
    static ref SPACE_ZEROS: HashMap<usize, String> =
        (0..=94).map(|i| (i, " ".repeat(i))).collect();
    static ref STRING_ZEROS: HashMap<usize, String> =
        (0..=94).map(|i| (i, "0".repeat(i))).collect();
}
