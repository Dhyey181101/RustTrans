
use std::collections::HashMap;
use std::ops::Range;
use std::str::FromStr;
use std::string::ToString;
use lazy_static::lazy_static;

const MOO_IO_ACH_CHECKING_CREDIT: u8 = 22;

#[derive(Debug)]
pub struct MoovIoAchEntryDetail {
    pub individual_name: String,
}

impl MoovIoAchEntryDetail {
    pub fn shr_individual_card_account_number_field(&self) -> String {
        MoovIoAchConverters {}.string_field(&self.individual_name, MOO_IO_ACH_CHECKING_CREDIT)
    }
}

pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: u8) -> String {
        let ln = s.chars().count() as u8;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = max - ln;
        let pad = MOO_IO_ACH_STRING_ZEROS.get(&m).unwrap_or_else(|| {
            panic!("pad not found for m: {}", m)
        });
        format!("{}{}", pad, s)
    }
}

lazy_static! {
    static ref MOO_IO_ACH_STRING_ZEROS: HashMap<u8, String> =
        (0..94).map(|i| (i, "0".repeat(i as usize))).collect();
}
