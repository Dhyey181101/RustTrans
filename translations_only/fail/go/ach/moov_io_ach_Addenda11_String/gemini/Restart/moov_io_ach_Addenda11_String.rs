
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::Write;

const MOOV_IO_ACH_RECORD_LENGTH: usize = 94;
const MOOV_IO_ACH_ENTRY_ADDENDA_POS: &str = "7";

#[derive(Default)]
struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    fn alpha_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = MOOV_IO_ACH_SPACE_ZEROS.get(&m).unwrap();
            format!("{}{}", s, pad)
        }
    }

    fn numeric_field(&self, n: i64, max: usize) -> String {
        let s = n.to_string();
        if s.len() > max {
            s[s.len() - max..].to_string()
        } else {
            let m = max - s.len();
            let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap();
            format!("{}{}", pad, s)
        }
    }
}

#[derive(Default)]
pub struct MoovIoAchAddenda11 {
    pub type_code: String,
    pub originator_name: String,
    pub originator_street_address: String,
    pub entry_detail_sequence_number: i64,
    pub converters: MoovIoAchConverters,
}

impl MoovIoAchAddenda11 {
    pub fn originator_name_field(&self) -> String {
        self.converters.alpha_field(&self.originator_name, 35)
    }

    pub fn originator_street_address_field(&self) -> String {
        self.converters.alpha_field(&self.originator_street_address, 35)
    }

    pub fn entry_detail_sequence_number_field(&self) -> String {
        self.converters.numeric_field(self.entry_detail_sequence_number, 7)
    }

    pub fn to_string(&self) -> String {
        let mut buf = String::with_capacity(MOOV_IO_ACH_RECORD_LENGTH);
        write!(
            &mut buf,
            "{}{}{}{}{}",
            MOOV_IO_ACH_ENTRY_ADDENDA_POS,
            self.type_code,
            self.originator_name_field(),
            self.originator_street_address_field(),
            "              "
        )
        .unwrap();
        write!(
            &mut buf,
            "{}",
            self.entry_detail_sequence_number_field()
        )
        .unwrap();
        buf
    }
}

lazy_static! {
    static ref MOOV_IO_ACH_SPACE_ZEROS: HashMap<usize, String> =
        (0..=94).map(|i| (i, " ".repeat(i))).collect();
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, String> =
        (0..=94).map(|i| (i, "0".repeat(i))).collect();
}
