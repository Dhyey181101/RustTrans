
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

    pub fn alpha_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let pad = SPACE_ZEROS.get(&m).unwrap();
        format!("{}{}", s, pad)
    }

    pub fn sequence_number_field(&self) -> String {
        self.numeric_field(self.sequence_number, 4)
    }

    pub fn numeric_field(&self, n: i32, max: usize) -> String {
        let s = n.to_string();
        if s.len() > max {
            return s[s.len() - max..].to_string();
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

impl FromStr for MoovIoAchAddenda05 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != MOOV_IO_ACH_RECORD_LENGTH {
            return Err(());
        }

        let type_code = &s[1..3];
        let payment_related_information = &s[3..83];
        let sequence_number = i32::from_str(&s[83..87]).unwrap();
        let entry_detail_sequence_number = i32::from_str(&s[87..94]).unwrap();

        Ok(MoovIoAchAddenda05 {
            type_code: type_code.to_string(),
            payment_related_information: payment_related_information.to_string(),
            sequence_number,
            entry_detail_sequence_number,
        })
    }
}

lazy_static::lazy_static! {
    static ref SPACE_ZEROS: HashMap<usize, String> =
        (0..=94).map(|i| (i, " ".repeat(i))).collect();
    static ref STRING_ZEROS: HashMap<usize, String> =
        (0..=94).map(|i| (i, "0".repeat(i))).collect();
}
