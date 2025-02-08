
use std::collections::HashMap;
use std::ops::Range;
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
        let binding = moov_io_ach_populate_map(m as usize, "0");
        let pad = binding.get(&(m as usize)).unwrap();
        format!("{}{}", pad, s)
    }
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}
