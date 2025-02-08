
use std::collections::HashMap;
use std::str;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, String>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

#[derive(Clone)]
struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s.chars().take(max).collect();
        }

        let m = max - ln;
        let pad = unsafe {
            MOOV_IO_ACH_STRING_ZEROS
                .as_ref()
                .unwrap()
                .get(&m)
                .unwrap()
                .clone()
        };
        pad + s
    }

    fn parse_string_field(&self, r: &str) -> String {
        r.trim().to_string()
    }
}

struct MoovIoAchIATEntryDetail {
    rdfi_identification: String,
    check_digit: String,
    converters: MoovIoAchConverters,
}

impl MoovIoAchIATEntryDetail {
    fn set_rdfi(&mut self, rdfi: &str) -> &mut Self {
        let s = self.converters.string_field(rdfi, 9);
        let bytes = s.as_bytes();
        self.rdfi_identification = self.converters.parse_string_field(str::from_utf8(&bytes[0..8]).unwrap());
        self.check_digit = self.converters.parse_string_field(str::from_utf8(&bytes[8..9]).unwrap());
        self
    }
}

fn main() {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
    }
}
