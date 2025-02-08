
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::Write;

const MOOv_IO_ACH_RECORD_LENGTH: usize = 94;
const MOOv_IO_ACH_ENTRY_ADDENDA_POS: &str = "7";

#[derive(Debug)]
pub struct MoovIoAchAddenda16 {
    pub type_code: String,
    pub receiver_city_state_province: String,
    pub receiver_country_postal_code: String,
    pub entry_detail_sequence_number: i32,
}

impl MoovIoAchAddenda16 {
    pub fn to_string(&self) -> String {
        let mut buf = String::with_capacity(MOOv_IO_ACH_RECORD_LENGTH);
        write!(buf, "{}", MOOv_IO_ACH_ENTRY_ADDENDA_POS).unwrap();
        write!(buf, "{}", self.type_code).unwrap();
        write!(buf, "{}", self.receiver_city_state_province_field()).unwrap();
        write!(buf, "{}", self.receiver_country_postal_code_field()).unwrap();
        write!(buf, "              ").unwrap();
        write!(buf, "{}", self.entry_detail_sequence_number_field()).unwrap();
        buf
    }

    pub fn receiver_city_state_province_field(&self) -> String {
        self.alpha_field(&self.receiver_city_state_province, 35)
    }

    pub fn alpha_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = SPACE_ZEROS.get(&m).unwrap();
            format!("{}{}", s, pad)
        }
    }

    pub fn receiver_country_postal_code_field(&self) -> String {
        self.alpha_field(&self.receiver_country_postal_code, 35)
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
