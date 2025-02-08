

use std::collections::HashMap;
use std::fmt;

const CHECKING_CREDIT: u8 = 22;

struct MoovIoAchEntryDetail {
    individual_name: String,
    // ... other fields ...
}

struct MoovIoAchConverters {}

impl MoovIoAchEntryDetail {
    fn set_shr_individual_card_account_number(&mut self, s: String) {
        self.individual_name = self.string_field(s, CHECKING_CREDIT as u32);
    }

    fn string_field(&self, s: String, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = (max - ln) as usize;
        let pad = get_zeros(m);
        format!("{}{}", pad, s)
    }
}

fn get_zeros(n: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..=n {
        out.insert(i, "0".repeat(i));
    }
    out[&n].to_string()
}

impl fmt::Display for MoovIoAchEntryDetail {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        // ... formatting and writing to `f` ...
        Ok(())
    }
}

fn main() {
    let mut ed = MoovIoAchEntryDetail {
        individual_name: String::from("John Doe"),
        // ... other fields ...
    };
    let s = String::from("123456789012345");
    ed.set_shr_individual_card_account_number(s);
    println!("{}", ed);
}

