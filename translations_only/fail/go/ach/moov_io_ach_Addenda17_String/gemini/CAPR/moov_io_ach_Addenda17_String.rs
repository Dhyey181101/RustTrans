
use std::collections::HashMap;
use std::fmt::Write;
use std::str::FromStr;
use lazy_static::lazy_static;

const MOO_IO_ACH_RECORD_LENGTH: usize = 94;
const MOO_IO_ACH_ENTRY_ADDENDA_POS: &str = "7";

#[derive(Debug, Default)]
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
        buf.push_str(MOO_IO_ACH_ENTRY_ADDENDA_POS);
        buf.push_str(&self.type_code);
        buf.push_str(&self.payment_related_information_field());
        buf.push_str(&self.sequence_number_field());
        buf.push_str(&self.entry_detail_sequence_number_field());
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
            let pad = &MOO_IO_ACH_SPACE_ZEROS[&m];
            format!("{}{}", s, pad)
        }
    }

    pub fn sequence_number_field(&self) -> String {
        self.numeric_field(self.sequence_number, 4)
    }

    fn numeric_field(&self, n: i32, max: usize) -> String {
        let s = n.to_string();
        let l = s.chars().count();
        if l > max {
            s[l - max..].to_string()
        } else {
            let m = max - l;
            let pad = &MOO_IO_ACH_STRING_ZEROS[&m];
            format!("{}{}", pad, s)
        }
    }

    pub fn entry_detail_sequence_number_field(&self) -> String {
        self.numeric_field(self.entry_detail_sequence_number, 7)
    }
}

lazy_static! {
    static ref MOO_IO_ACH_SPACE_ZEROS: HashMap<usize, String> =
        (0..94).map(|i| (i, " ".repeat(i))).collect();
    static ref MOO_IO_ACH_STRING_ZEROS: HashMap<usize, String> =
        (0..94).map(|i| (i, "0".repeat(i))).collect();
}
