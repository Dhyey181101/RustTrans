
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug, Clone)]
pub struct MoovIoAchIATEntryDetail {
    pub r_d_f_i_identification: String,
    pub check_digit: String,
}

impl MoovIoAchIATEntryDetail {
    pub fn set_r_d_f_i(&mut self, r_d_f_i: &str) -> &mut Self {
        let s = self.string_field(r_d_f_i, 9);
        self.r_d_f_i_identification = self.parse_string_field(&s[..8]);
        self.check_digit = self.parse_string_field(&s[8..9]);
        self
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let pad = moov_io_ach_string_zeros.get(&m).unwrap();
        format!("{}{}", pad, s)
    }

    fn parse_string_field(&self, r: &str) -> String {
        r.trim().to_string()
    }
}

lazy_static! {
    static ref moov_io_ach_string_zeros: HashMap<usize, String> =
        (0..94).map(|i| (i, format!("{:0width$}", "", width = i))).collect();
}
