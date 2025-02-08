
use std::collections::HashMap;

lazy_static::lazy_static! {
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
    converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchEntryDetail {
    fn new() -> Self {
        MoovIoAchEntryDetail {
            rdfi_identification: String::new(),
            check_digit: String::new(),
            converters: Box::new(MoovIoAchConverters {}),
        }
    }

    fn set_rdfi(&mut self, rdfi: &str) -> &mut Self {
        let s = self.converters.string_field(rdfi, 9);
        self.rdfi_identification = self.converters.parse_string_field(&s[..8]);
        self.check_digit = self.converters.parse_string_field(&s[8..9]);
        self
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s.chars().take(max).collect();
        }

        // Pad with preallocated string
        let m = max - ln;
        if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
            return pad.clone() + s;
        }
        // slow path
        "0".repeat(m) + s
    }

    fn parse_string_field(&self, r: &str) -> String {
        r.trim().to_string()
    }
}

fn main() {
    // Example usage
    let mut entry_detail = MoovIoAchEntryDetail::new();
    entry_detail.set_rdfi("12345678");
    println!("RDFI Identification: {}", entry_detail.rdfi_identification);
    println!("Check Digit: {}", entry_detail.check_digit);
}
