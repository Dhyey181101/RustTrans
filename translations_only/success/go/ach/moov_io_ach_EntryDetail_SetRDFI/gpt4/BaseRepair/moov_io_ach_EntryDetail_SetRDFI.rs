
use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, String> = moov_io_ach_populate_map(94, "0");
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchEntryDetail {
    rdfi_identification: String,
    check_digit: String,
    converters: MoovIoAchConverters,
}

impl MoovIoAchEntryDetail {
    fn set_rdfi(mut self, rdfi: String) -> Box<Self> {
        let s = self.converters.string_field(&rdfi, 9);
        self.rdfi_identification = self.converters.parse_string_field(&s[..8]);
        self.check_digit = self.converters.parse_string_field(&s[8..9]);
        Box::new(self)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s.chars().take(max).collect();
        }

        let m = max - ln;
        if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
            return format!("{}{}", pad, s);
        }

        "0".repeat(m) + s
    }

    fn parse_string_field(&self, r: &str) -> String {
        r.trim().to_string()
    }
}
