

use std::collections::HashMap;
use std::fmt;

const CHECKING_CREDIT: u8 = 22;

struct MoovIoAchEntryDetail {
    individual_name: String,
}

impl MoovIoAchEntryDetail {
    fn set_shr_individual_card_account_number(&mut self, s: String) {
        self.individual_name = MoovIoAchConverters::string_field(s, CHECKING_CREDIT as u32);
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(s: String, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = (max - ln) as usize;
        let pad = MoovIoAchConverters::get_zeros(m);
        format!("{}{}", pad, s)
    }

    fn get_zeros(n: usize) -> String {
        let mut out = String::new();
        for _ in 0..=n {
            out.push('0');
        }
        out
    }
}

impl fmt::Display for MoovIoAchEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "individualName: {}", self.individual_name)
    }
}

fn main() {
    let mut ed = MoovIoAchEntryDetail {
        individual_name: String::from("test"),
    };
    ed.set_shr_individual_card_account_number(String::from("12345678901234567890123"));
    println!("{}", ed);
}

