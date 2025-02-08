
use std::collections::HashMap;
use std::fmt::Write;
use std::str::FromStr;
use lazy_static::lazy_static;

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
    pub fn to_string(&self) -> String {
        let mut buf = String::with_capacity(MOOV_IO_ACH_RECORD_LENGTH);
        write!(buf, "{}{}{}{}{}", MOOV_IO_ACH_ENTRY_ADDENDA_POS, self.type_code, self.receiver_id_number_field(), self.receiver_street_address_field(), "                                  ").unwrap();
        write!(buf, "{}", self.entry_detail_sequence_number_field()).unwrap();
        buf
    }

    pub fn receiver_id_number_field(&self) -> String {
        self.alpha_field(&self.receiver_id_number, 15)
    }

    fn alpha_field(&self, s: &Option<String>, max: usize) -> String {
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

lazy_static! {
    static ref SPACE_ZEROS: HashMap<usize, String> = populate_map(94, " ");
    static ref STRING_ZEROS: HashMap<usize, String> = populate_map(94, "0");
}

fn populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}
