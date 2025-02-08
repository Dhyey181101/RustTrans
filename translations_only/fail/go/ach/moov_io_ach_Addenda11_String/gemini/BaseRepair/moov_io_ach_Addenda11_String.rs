
use std::collections::HashMap;
use std::fmt::Write;
use std::str::FromStr;
use lazy_static::lazy_static;

const MOOV_IO_ACH_RECORD_LENGTH: usize = 94;
const MOOV_IO_ACH_ENTRY_ADDENDA_POS: &str = "7";

#[derive(Debug, Default)]
pub struct MoovIoAchAddenda11 {
    pub type_code: String,
    pub originator_name: String,
    pub originator_street_address: String,
    pub entry_detail_sequence_number: i32,
}

impl MoovIoAchAddenda11 {
    pub fn to_string(&self) -> String {
        let mut buf = String::with_capacity(MOOV_IO_ACH_RECORD_LENGTH);
        write!(
            &mut buf,
            "{}{}{}{}{}{}",
            MOOV_IO_ACH_ENTRY_ADDENDA_POS,
            self.type_code,
            self.originator_name_field(),
            self.originator_street_address_field(),
            "              ",
            self.entry_detail_sequence_number_field()
        )
        .unwrap();
        buf
    }

    pub fn originator_name_field(&self) -> String {
        let mut pad = String::new();
        let ln = self.originator_name.chars().count();
        if ln > 35 {
            pad = self.originator_name[..35].to_string();
        } else {
            pad = format!("{}{}", self.originator_name, " ".repeat(35 - ln));
        }
        pad
    }

    pub fn originator_street_address_field(&self) -> String {
        let mut pad = String::new();
        let ln = self.originator_street_address.chars().count();
        if ln > 35 {
            pad = self.originator_street_address[..35].to_string();
        } else {
            pad = format!("{}{}", self.originator_street_address, " ".repeat(35 - ln));
        }
        pad
    }

    pub fn entry_detail_sequence_number_field(&self) -> String {
        let mut pad = String::new();
        let s = self.entry_detail_sequence_number.to_string();
        if s.len() > 7 {
            pad = s[s.len() - 7..].to_string();
        } else {
            pad = format!("{}{}", "0".repeat(7 - s.len()), s);
        }
        pad
    }
}

lazy_static! {
    static ref SPACE_ZEROS: HashMap<usize, String> =
        (0..=94).map(|i| (i, " ".repeat(i))).collect();
    static ref STRING_ZEROS: HashMap<usize, String> =
        (0..=94).map(|i| (i, "0".repeat(i))).collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        let addenda11 = MoovIoAchAddenda11 {
            type_code: "11".to_string(),
            originator_name: "ACME CORPORATION".to_string(),
            originator_street_address: "123 MAIN STREET".to_string(),
            entry_detail_sequence_number: 1234567,
        };
        assert_eq!(
            addenda11.to_string(),
            "711ACME CORPORATION123 MAIN STREET              1234567"
        );
    }

    #[test]
    fn test_originator_name_field() {
        let addenda11 = MoovIoAchAddenda11 {
            originator_name: "ACME CORPORATION".to_string(),
            ..Default::default()
        };
        assert_eq!(addenda11.originator_name_field(), "ACME CORPORATION              ");
    }

    #[test]
    fn test_originator_street_address_field() {
        let addenda11 = MoovIoAchAddenda11 {
            originator_street_address: "123 MAIN STREET".to_string(),
            ..Default::default()
        };
        assert_eq!(
            addenda11.originator_street_address_field(),
            "123 MAIN STREET              "
        );
    }

    #[test]
    fn test_entry_detail_sequence_number_field() {
        let addenda11 = MoovIoAchAddenda11 {
            entry_detail_sequence_number: 1234567,
            ..Default::default()
        };
        assert_eq!(addenda11.entry_detail_sequence_number_field(), "1234567");
    }
}
