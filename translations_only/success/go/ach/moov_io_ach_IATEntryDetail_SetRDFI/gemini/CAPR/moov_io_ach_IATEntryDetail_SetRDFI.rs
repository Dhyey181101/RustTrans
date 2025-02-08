
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct MoovIoAchIATEntryDetail {
    pub r_d_f_i_identification: String,
    pub check_digit: String,
}

impl MoovIoAchIATEntryDetail {
    pub fn set_r_d_f_i(&mut self, rdfi: &str) -> &mut Self {
        let s = rdfi.chars().take(8).collect::<String>();
        self.r_d_f_i_identification = s;
        self.check_digit = rdfi.chars().nth(8).unwrap().to_string();
        self
    }
}

pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = max - ln;
        let pad = moov_io_ach_string_zeros.get(&m).unwrap();
        format!("{}{}", pad, s)
    }

    pub fn parse_string_field(&self, r: &str) -> String {
        r.trim().to_string()
    }
}

lazy_static! {
    static ref moov_io_ach_string_zeros: HashMap<u32, String> = moov_io_ach_populate_map(94, "0");
}

fn moov_io_ach_populate_map(max: u32, zero: &str) -> HashMap<u32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}
