

use std::collections::HashMap;
use std::fmt;

struct MoovIoAchConverters {
    moov_io_ach_string_zeros: HashMap<usize, &'static str>,
}

impl MoovIoAchConverters {
    fn new() -> Self {
        MoovIoAchConverters {
            moov_io_ach_string_zeros: HashMap::from([(1, "0"), (2, "00"), (3, "000")]),
        }
    }
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.len();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = self.moov_io_ach_string_zeros.get(&m).unwrap_or(&"");
            pad.to_string() + s
        }
    }
    fn parse_string_field(&self, r: &str) -> String {
        r.trim().to_string()
    }
}

impl Default for MoovIoAchConverters {
    fn default() -> Self {
        MoovIoAchConverters::new()
    }
}

struct MoovIoAchEntryDetail {
    rdfi_identification: String,
    check_digit: String,
    // ... other fields ...
    moov_io_ach_converters: MoovIoAchConverters,
}

impl MoovIoAchEntryDetail {
    fn set_rdfi(&mut self, rdfi: &str) -> &mut Self {
        let s = self.moov_io_ach_converters.string_field(rdfi, 9);
        self.rdfi_identification = self.moov_io_ach_converters.parse_string_field(&s[..8]);
        self.check_digit = self.moov_io_ach_converters.parse_string_field(&s[8..9]);
        self
    }
}

impl Default for MoovIoAchEntryDetail {
    fn default() -> Self {
        MoovIoAchEntryDetail {
            rdfi_identification: String::new(),
            check_digit: String::new(),
            moov_io_ach_converters: MoovIoAchConverters::default(),
        }
    }
}

impl fmt::Display for MoovIoAchEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // ... implement display ...
        Ok(())
    }
}

fn main() {
    let mut ed = MoovIoAchEntryDetail::default();
    ed.set_rdfi("123456789");
    println!("{}", ed);
}

