

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug)]
pub struct MoovIoAchEntryDetail {
    pub r_d_f_i_identification: String,
}

impl MoovIoAchEntryDetail {
    pub fn r_d_f_i_identification_field(&self) -> String {
        self.string_field(self.r_d_f_i_identification.clone(), 8)
    }

    fn string_field(&self, s: String, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = max - ln;
        let pad = moov_io_ach_string_zeros.get(&m).unwrap();
        return format!("{}{}", pad, s);
    }
}

lazy_static! {
    static ref moov_io_ach_string_zeros: HashMap<u32, String> =
        moov_io_ach_populate_map(94, "0");
}

fn moov_io_ach_populate_map(max: u32, zero: &str) -> HashMap<u32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

