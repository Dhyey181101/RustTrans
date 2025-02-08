
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::Write;
use std::str::FromStr;

const MOOV_IO_ACH_RECORD_LENGTH: usize = 94;
const MOOV_IO_ACH_ENTRY_ADDENDA_POS: &str = "7";

#[derive(Debug, Default)]
pub struct MoovIoAchAddenda17 {
    pub type_code: String,
    pub payment_related_information: String,
    pub sequence_number: i32,
    pub entry_detail_sequence_number: i32,
}

impl MoovIoAchAddenda17 {
    pub fn to_string(&self) -> String {
        let mut buf = String::with_capacity(MOOV_IO_ACH_RECORD_LENGTH);
        write!(buf, "{}", MOOV_IO_ACH_ENTRY_ADDENDA_POS).unwrap();
        write!(buf, "{}", self.type_code).unwrap();
        write!(buf, "{}", self.payment_related_information_field()).unwrap();
        write!(buf, "{}", self.sequence_number_field()).unwrap();
        write!(buf, "{}", self.entry_detail_sequence_number_field()).unwrap();
        buf
    }

    pub fn payment_related_information_field(&self) -> String {
        let s = &self.payment_related_information;
        let ln = s.chars().count();
        if ln > 80 {
            s[..80].to_string()
        } else {
            let m = 80 - ln;
            let pad = SPACE_ZEROS.get(&m).unwrap();
            format!("{}{}", s, pad)
        }
    }

    pub fn sequence_number_field(&self) -> String {
        let n = self.sequence_number;
        let s = n.to_string();
        let l = s.chars().count();
        if l > 4 {
            s[l - 4..].to_string()
        } else {
            let m = 4 - l;
            let pad = STRING_ZEROS.get(&m).unwrap();
            format!("{}{}", pad, s)
        }
    }

    pub fn entry_detail_sequence_number_field(&self) -> String {
        let n = self.entry_detail_sequence_number;
        let s = n.to_string();
        let l = s.chars().count();
        if l > 7 {
            s[l - 7..].to_string()
        } else {
            let m = 7 - l;
            let pad = STRING_ZEROS.get(&m).unwrap();
            format!("{}{}", pad, s)
        }
    }
}

impl FromStr for MoovIoAchAddenda17 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut addenda17 = MoovIoAchAddenda17::default();
        addenda17.type_code = s[1..3].to_string();
        addenda17.payment_related_information = s[3..83].to_string();
        addenda17.sequence_number = s[83..87].parse().unwrap();
        addenda17.entry_detail_sequence_number = s[87..94].parse().unwrap();
        Ok(addenda17)
    }
}

lazy_static! {
    static ref SPACE_ZEROS: HashMap<usize, String> = (0..=94).map(|i| (i, " ".repeat(i))).collect();
    static ref STRING_ZEROS: HashMap<usize, String> = (0..=94).map(|i| (i, "0".repeat(i))).collect();
}
