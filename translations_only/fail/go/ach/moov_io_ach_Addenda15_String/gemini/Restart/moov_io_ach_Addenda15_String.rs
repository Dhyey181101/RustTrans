
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

const MOOV_IO_ACH_RECORD_LENGTH: usize = 94;
const MOOV_IO_ACH_ENTRY_ADDENDA_POS: &str = "7";

#[derive(Debug, Clone)]
pub struct MoovIoAchAddenda15 {
    pub type_code: String,
    pub receiver_id_number: Option<String>,
    pub receiver_street_address: String,
    pub entry_detail_sequence_number: i32,
}

impl Display for MoovIoAchAddenda15 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            MOOV_IO_ACH_ENTRY_ADDENDA_POS,
            self.type_code,
            self.receiver_id_number_field(),
            self.receiver_street_address_field(),
            self.entry_detail_sequence_number_field()
        )
    }
}

impl MoovIoAchAddenda15 {
    pub fn receiver_id_number_field(&self) -> String {
        self.alpha_field(&self.receiver_id_number, 15)
    }

    pub fn alpha_field(&self, s: &Option<String>, max: usize) -> String {
        let ln = s.as_ref().map_or(0, |s| s.chars().count());
        if ln > max {
            s.as_ref().unwrap()[..max].to_string()
        } else {
            let m = max - ln;
            let pad = SPACE_ZEROS.get(&m).unwrap();
            format!("{}{}", s.as_ref().unwrap(), pad)
        }
    }

    pub fn receiver_street_address_field(&self) -> String {
        self.alpha_field(&Some(self.receiver_street_address.clone()), 35)
    }

    pub fn entry_detail_sequence_number_field(&self) -> String {
        self.numeric_field(self.entry_detail_sequence_number, 7)
    }

    pub fn numeric_field(&self, n: i32, max: usize) -> String {
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

impl FromStr for MoovIoAchAddenda15 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        Ok(MoovIoAchAddenda15 {
            type_code: chars.by_ref().take(2).collect(),
            receiver_id_number: Some(chars.by_ref().take(15).collect()),
            receiver_street_address: chars.by_ref().take(35).collect(),
            entry_detail_sequence_number: chars.by_ref().take(7).collect::<String>().parse().unwrap(),
        })
    }
}

lazy_static::lazy_static! {
    static ref SPACE_ZEROS: HashMap<usize, String> = (0..MOOV_IO_ACH_RECORD_LENGTH)
        .map(|i| (i, " ".repeat(i)))
        .collect();
    static ref STRING_ZEROS: HashMap<usize, String> = (0..MOOV_IO_ACH_RECORD_LENGTH)
        .map(|i| (i, "0".repeat(i)))
        .collect();
}
