
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug)]
pub struct BatchControl {
    pub odfi_identification: String,
}

impl BatchControl {
    pub fn odfi_identification_field(&self) -> String {
        self.string_field(&self.odfi_identification, 8)
    }

    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = max - ln;
        let pad = moov_io_ach_string_zeros.get(&(m as i32)).unwrap();
        pad.to_string() + s
    }
}

lazy_static! {
    static ref moov_io_ach_string_zeros: HashMap<i32, String> =
        moov_io_ach_populate_map(94, "0");
}

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::with_capacity(max as usize);
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}
