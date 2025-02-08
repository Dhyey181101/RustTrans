
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

const MOOV_IO_ACH_RECORD_LENGTH: usize = 94;
const MOOV_IO_ACH_ENTRY_ADDENDA_POS: &str = "7";

#[derive(Debug, Clone)]
pub struct MoovIoAchAddenda05 {
    pub type_code: String,
    pub payment_related_information: String,
    pub sequence_number: i32,
    pub entry_detail_sequence_number: i32,
}

impl MoovIoAchAddenda05 {
    pub fn new(
        type_code: String,
        payment_related_information: String,
        sequence_number: i32,
        entry_detail_sequence_number: i32,
    ) -> Self {
        MoovIoAchAddenda05 {
            type_code,
            payment_related_information,
            sequence_number,
            entry_detail_sequence_number,
        }
    }
}

impl fmt::Display for MoovIoAchAddenda05 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            MOOV_IO_ACH_ENTRY_ADDENDA_POS,
            self.type_code,
            self.payment_related_information_field(),
            self.sequence_number_field(),
            self.entry_detail_sequence_number_field()
        )
    }
}

impl MoovIoAchAddenda05 {
    pub fn payment_related_information_field(&self) -> String {
        self.alpha_field(&self.payment_related_information, 80)
    }

    fn alpha_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let mut pad = String::new();
            for _ in 0..m {
                pad.push(' ');
            }
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
            let mut pad = String::new();
            for _ in 0..m {
                pad.push('0');
            }
            format!("{}{}", pad, s)
        }
    }

    pub fn entry_detail_sequence_number_field(&self) -> String {
        self.numeric_field(self.entry_detail_sequence_number, 7)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moov_io_ach_addenda05() {
        let addenda05 = MoovIoAchAddenda05::new(
            "05".to_string(),
            "Payment Related Information".to_string(),
            1,
            1234567,
        );
        assert_eq!(
            addenda05.to_string(),
            "705Payment Related Information00011234567"
        );
    }
}
