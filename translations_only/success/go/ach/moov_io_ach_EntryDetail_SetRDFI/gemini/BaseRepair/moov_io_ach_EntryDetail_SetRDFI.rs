
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug, Clone)]
pub struct MoovIoAchEntryDetail {
    pub r_d_f_i_identification: String,
    pub check_digit: String,
}

impl MoovIoAchEntryDetail {
    pub fn set_r_d_f_i(&mut self, r_d_f_i: &str) -> &mut Self {
        let s = r_d_f_i.to_string();
        self.r_d_f_i_identification = s[..8].to_string();
        self.check_digit = s[8..9].to_string();
        self
    }
}

#[derive(Debug, Clone)]
pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = max - ln;
        let pad = MOOv_IO_ACH_STRING_ZEROS.get(&(m as i32)).unwrap();
        format!("{}{}", pad, s)
    }

    pub fn parse_string_field(&self, r: &str) -> String {
        r.trim().to_string()
    }
}

lazy_static! {
    static ref MOOv_IO_ACH_STRING_ZEROS: HashMap<i32, String> =
        (0..94).map(|i| (i, "0".repeat(i as usize))).collect();
}
