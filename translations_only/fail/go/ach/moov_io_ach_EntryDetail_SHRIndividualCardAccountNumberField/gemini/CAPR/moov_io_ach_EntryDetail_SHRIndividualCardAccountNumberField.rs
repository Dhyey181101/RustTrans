
use lazy_static::lazy_static;

use std::collections::HashMap;

const MOO_IO_ACH_CHECKING_CREDIT: u8 = 22;

struct MoovIoAchEntryDetail {
    individual_name: String,
}

impl MoovIoAchEntryDetail {
    fn shr_individual_card_account_number_field(&self) -> String {
        let m = self.individual_name.chars().count() as u32;
        MoovIoAchConverters::string_field(self.individual_name.clone(), m)
    }
}

struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    fn string_field(s: String, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = max - ln;
        let pad = MOO_IO_ACH_STRING_ZEROS.get(&m).unwrap();
        pad.to_string() + &s
    }
}

lazy_static! {
    static ref MOO_IO_ACH_STRING_ZEROS: HashMap<u32, String> = moov_io_ach_populate_map(94, "0");
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<u32, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i as u32, zero.repeat(i));
    }
    out
}
