
use std::collections::HashMap;
use std::fmt::Write;

const MOO_IO_ACH_RECORD_LENGTH: usize = 94;
const MOO_IO_ACH_ENTRY_ADDENDA_POS: &str = "7";

#[derive(Debug, Default)]
pub struct MoovIoAchAddenda05 {
    pub type_code: String,
    pub payment_related_information: String,
    pub sequence_number: i32,
    pub entry_detail_sequence_number: i32,
}

impl MoovIoAchAddenda05 {
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
        self.alpha_field(&self.payment_related_information, 80)
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

    pub fn sequence_number_field(&self) -> String {
        self.numeric_field(self.sequence_number, 4)
    }

    fn numeric_field(&self, n: i32, max: usize) -> String {
        let s = n.to_string();
        if s.len() > max {
            s[s.len() - max..].to_string()
        } else {
            let m = max - s.len();
            let pad = STRING_ZEROS.get(&m).unwrap();
            format!("{}{}", pad, s)
        }
    }

    pub fn entry_detail_sequence_number_field(&self) -> String {
        self.numeric_field(self.entry_detail_sequence_number, 7)
    }
}

lazy_static::lazy_static! {
    static ref SPACE_ZEROS: HashMap<usize, String> = (0..=94).map(|i| (i, " ".repeat(i))).collect();
    static ref STRING_ZEROS: HashMap<usize, String> = (0..=94).map(|i| (i, "0".repeat(i))).collect();
}
