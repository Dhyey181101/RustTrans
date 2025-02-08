

use std::collections::HashMap;
use std::fmt::Write;
use std::str::FromStr;

const MOOv_IO_ACH_RECORD_LENGTH: usize = 94;
const MOOv_IO_ACH_ENTRY_ADDENDA_POS: &str = "7";

#[derive(Debug, Default)]
pub struct MoovIoAchAddenda12 {
    pub type_code: String,
    pub originator_city_state_province: String,
    pub originator_country_postal_code: String,
    pub entry_detail_sequence_number: i32,
}

impl MoovIoAchAddenda12 {
    pub fn new(
        type_code: String,
        originator_city_state_province: String,
        originator_country_postal_code: String,
        entry_detail_sequence_number: i32,
    ) -> Self {
        Self {
            type_code,
            originator_city_state_province,
            originator_country_postal_code,
            entry_detail_sequence_number,
        }
    }

    pub fn to_string(&self) -> String {
        let mut buf = String::with_capacity(MOOv_IO_ACH_RECORD_LENGTH);
        write!(
            &mut buf,
            "{}{}{}{}{}{}",
            MOOv_IO_ACH_ENTRY_ADDENDA_POS,
            self.type_code,
            self.originator_city_state_province_field(),
            self.originator_country_postal_code_field(),
            "              ",
            self.entry_detail_sequence_number_field()
        )
        .unwrap();
        buf
    }

    fn originator_city_state_province_field(&self) -> String {
        self.alpha_field(&self.originator_city_state_province, 35)
    }

    fn alpha_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = " ".repeat(m);
            format!("{}{}", s, pad)
        }
    }

    fn originator_country_postal_code_field(&self) -> String {
        self.alpha_field(&self.originator_country_postal_code, 35)
    }

    fn entry_detail_sequence_number_field(&self) -> String {
        self.numeric_field(self.entry_detail_sequence_number, 7)
    }

    fn numeric_field(&self, n: i32, max: usize) -> String {
        let s = n.to_string();
        let l = s.chars().count();
        if l > max {
            s[l - max..].to_string()
        } else {
            let m = max - l;
            let pad = "0".repeat(m);
            format!("{}{}", pad, s)
        }
    }
}

static SPACE_ZEROS: [&str; 95] = [" "; 95];
static STRING_ZEROS: [&str; 95] = ["0"; 95];


