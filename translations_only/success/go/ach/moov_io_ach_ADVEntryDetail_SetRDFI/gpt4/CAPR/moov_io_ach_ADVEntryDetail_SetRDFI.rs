
use std::collections::HashMap;

lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, String> = moov_io_ach_populate_map(94, "0");
}

#[derive(Clone)]
struct MoovIoAchADVEntryDetail {
    rdfi_identification: String,
    check_digit: String,
    moov_io_a_ch_converters: Box<MoovIoAchConverters>,
}

#[derive(Clone)]
struct MoovIoAchConverters;

impl MoovIoAchADVEntryDetail {
    fn set_rdfi(&mut self, rdfi: String) -> Box<Self> {
        let s = self.moov_io_a_ch_converters.string_field(&rdfi, 9);
        self.rdfi_identification = self.moov_io_a_ch_converters.parse_string_field(&s[..8]);
        self.check_digit = self.moov_io_a_ch_converters.parse_string_field(&s[8..9]);
        Box::new(self.clone())
    }
}

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

        format!("{}{}", "0".repeat(m), s)
    }

    fn parse_string_field(&self, r: &str) -> String {
        r.trim().to_string()
    }
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

#[macro_use]
extern crate lazy_static;

fn main() {}
