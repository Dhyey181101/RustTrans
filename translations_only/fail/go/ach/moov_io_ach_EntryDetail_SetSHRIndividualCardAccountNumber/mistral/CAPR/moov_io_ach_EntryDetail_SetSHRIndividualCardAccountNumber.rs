

use std::collections::HashMap;
use std::fmt;
use once_cell::sync::Lazy;

const MOOV_IO_ACH_CHECKING_CREDIT: u8 = 22;

struct MoovIoAchEntryDetail {
    individual_name: String,
    // ... other fields ...
    moov_io_ach_converters: MoovIoAchConverters,
}

impl MoovIoAchEntryDetail {
    fn set_shr_individual_card_account_number(&mut self, s: String) {
        self.individual_name = self.string_field(s, MOOV_IO_ACH_CHECKING_CREDIT as u32);
    }

    fn string_field(&self, s: String, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = (max - ln) as usize;
        if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
            return pad.clone();
        }

        "0".repeat(m) + &s
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
        let mut out = HashMap::new();
        for i in 0..max {
            out.insert(i, "0".repeat(i));
        }
        out
    }
}

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    MoovIoAchConverters::populate_map(94, "0")
});

impl fmt::Display for MoovIoAchEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // ... format and return the struct as a string ...
        write!(f, "{}", self.individual_name)
    }
}

