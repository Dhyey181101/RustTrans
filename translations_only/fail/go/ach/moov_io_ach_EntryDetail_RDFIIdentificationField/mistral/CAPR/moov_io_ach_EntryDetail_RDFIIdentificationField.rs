

use std::collections::HashMap;
use std::fmt;

const ZERO: char = '0';

struct MoovIoAchEntryDetail {
    rdfi_identification: String,
}

struct MoovIoAchConverters;

impl MoovIoAchEntryDetail {
    fn rdfi_identification_field(&self) -> String {
        MoovIoAchConverters::string_field(&self.rdfi_identification, 8)
    }
}

impl MoovIoAchConverters {
    fn string_field(s: &String, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[0..max as usize].to_string();
        }

        let m = (max - ln) as usize;
        let pad = Self::get_zeros(m);
        pad.chars().chain(s.chars()).collect()
    }

    fn get_zeros(n: usize) -> String {
        let mut zeros = HashMap::new();
        zeros.insert(1, "0".to_string());

        for i in 2..=n {
            let zero_str = zeros.get(&(i - 1)).unwrap().clone();
            zeros.insert(i, format!("{}{}", zero_str, ZERO));
        }

        zeros.get(&n).unwrap().clone()
    }
}

impl fmt::Display for MoovIoAchEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.rdfi_identification_field())
    }
}

fn main() {
    let entry_detail = MoovIoAchEntryDetail {
        rdfi_identification: "123456789".to_string(),
    };

    println!("{}", entry_detail);
}

