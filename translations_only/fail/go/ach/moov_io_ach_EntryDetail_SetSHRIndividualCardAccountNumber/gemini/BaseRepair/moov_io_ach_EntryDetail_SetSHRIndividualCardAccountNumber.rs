
use std::collections::HashMap;
use std::ops::Deref;
use std::str::FromStr;
use std::string::ToString;

const MOOV_IO_ACH_CHECKING_CREDIT: u8 = 22;

struct MoovIoAchEntryDetail {
    individual_name: String,
}

impl MoovIoAchEntryDetail {
    fn set_shr_individual_card_account_number(&mut self, s: &str) {
        self.individual_name = s.chars().take(MOOV_IO_ACH_CHECKING_CREDIT as usize).collect();
    }
}

struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = max - ln;
        let binding = moov_io_ach_populate_map(m as i32, "0");
        let pad = binding.get(&(m as i32)).unwrap();
        format!("{}{}", pad, s)
    }
}

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    (0..max).map(|i| (i, zero.repeat(i as usize))).collect()
}

fn main() {
    let mut ed = MoovIoAchEntryDetail {
        individual_name: String::new(),
    };
    ed.set_shr_individual_card_account_number("12345678901234567890");
    println!("{}", ed.individual_name);
}
