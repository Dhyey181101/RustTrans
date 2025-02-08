

use std::collections::HashMap;
use std::fmt;
use once_cell::sync::Lazy;

const MOOV_IO_ACH_CHECKING_CREDIT: u8 = 22;

struct MoovIoAchEntryDetail {
    individual_name: String,
    // ... other fields ...
    moov_io_ach_converters: MoovIoAchConverters,
}

struct MoovIoAchConverters;

impl MoovIoAchEntryDetail {
    fn shr_individual_card_account_number_field(&self) -> String {
        self.string_field(&self.individual_name, MOOV_IO_ACH_CHECKING_CREDIT)
    }

    fn string_field(&self, s: &str, max: u8) -> String {
        let ln = s.len() as u32;
        let max = max as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = (max - ln) as usize;
        if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
            return format!("{}{}", pad, s);
        }

        "0".repeat(m as usize) + s
    }
}

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> =
    Lazy::new(|| moov_io_ach_populate_map(100, "0"));

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, format!("{}", zero.repeat(i)));
    }
    out
}

impl fmt::Display for MoovIoAchEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // ... format and return the EntryDetail struct ...
        unimplemented!()
    }
}

