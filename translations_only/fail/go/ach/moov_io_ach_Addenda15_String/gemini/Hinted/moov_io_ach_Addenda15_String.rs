
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
    pub fn new(
        type_code: String,
        receiver_id_number: Option<String>,
        receiver_street_address: String,
        entry_detail_sequence_number: i32,
    ) -> Self {
        Self {
            type_code,
            receiver_id_number,
            receiver_street_address,
            entry_detail_sequence_number,
        }
    }

    pub fn to_string(&self) -> String {
        let mut buf = String::with_capacity(MOOV_IO_ACH_RECORD_LENGTH);
        let receiver_id_number_field = self.receiver_id_number_field();
        let receiver_street_address_field = self.receiver_street_address_field();
        let entry_detail_sequence_number_field = self.entry_detail_sequence_number_field();
        write!(
            &mut buf,
            "{}{}{}{}{}{}",
            MOOV_IO_ACH_ENTRY_ADDENDA_POS,
            self.type_code,
            receiver_id_number_field,
            receiver_street_address_field,
            "                                  ",
            entry_detail_sequence_number_field
        )
        .unwrap();
        buf
    }

    pub fn receiver_id_number_field(&self) -> String {
        self.alpha_field(&self.receiver_id_number.as_deref().unwrap_or_default(), 15)
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

    pub fn receiver_street_address_field(&self) -> String {
        self.alpha_field(&self.receiver_street_address, 35)
    }

    pub fn entry_detail_sequence_number_field(&self) -> String {
        self.numeric_field(self.entry_detail_sequence_number, 7)
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
}

lazy_static::lazy_static! {
    static ref SPACE_ZEROS: HashMap<usize, String> =
        (0..=MOOV_IO_ACH_RECORD_LENGTH).map(|i| (i, " ".repeat(i))).collect();
    static ref STRING_ZEROS: HashMap<usize, String> =
        (0..=MOOV_IO_ACH_RECORD_LENGTH).map(|i| (i, "0".repeat(i))).collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        let addenda15 = MoovIoAchAddenda15::new(
            "15".to_string(),
            Some("123456789012345".to_string()),
            "123 Main Street".to_string(),
            1234567,
        );
        assert_eq!(
            addenda15.to_string(),
            "715123456789012345123 Main Street                                  1234567"
        );
    }

    #[test]
    fn test_receiver_id_number_field() {
        let addenda15 = MoovIoAchAddenda15::new(
            "15".to_string(),
            Some("123456789012345".to_string()),
            "123 Main Street".to_string(),
            1234567,
        );
        assert_eq!(addenda15.receiver_id_number_field(), "123456789012345");
    }

    #[test]
    fn test_receiver_street_address_field() {
        let addenda15 = MoovIoAchAddenda15::new(
            "15".to_string(),
            Some("123456789012345".to_string()),
            "123 Main Street".to_string(),
            1234567,
        );
        assert_eq!(addenda15.receiver_street_address_field(), "123 Main Street");
    }

    #[test]
    fn test_entry_detail_sequence_number_field() {
        let addenda15 = MoovIoAchAddenda15::new(
            "15".to_string(),
            Some("123456789012345".to_string()),
            "123 Main Street".to_string(),
            1234567,
        );
        assert_eq!(addenda15.entry_detail_sequence_number_field(), "1234567");
    }
}
